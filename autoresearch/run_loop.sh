#!/usr/bin/env bash
# run_loop.sh — pptx2html-rs autoresearch experiment loop
#
# Usage:
#   ./autoresearch/run_loop.sh --phase 01_color_fidelity [--max-iterations 100]
#
# Prerequisites:
#   - cargo (Rust toolchain)
#   - python3 with evaluate/evaluate_fidelity.py dependencies
#   - git (clean working tree recommended)
#
# Exit codes:
#   0  All iterations completed or manual stop (SIGINT/SIGTERM)
#   1  General error
#   2  Invalid arguments
#   3  Missing prerequisites
#   4  Project root detection failure

set -Eeuo pipefail

# inherit_errexit requires Bash 4.4+; skip gracefully on older versions
if (( BASH_VERSINFO[0] > 4 || (BASH_VERSINFO[0] == 4 && BASH_VERSINFO[1] >= 4) )); then
    shopt -s inherit_errexit
fi

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
readonly VERSION="1.0.0"
readonly EXPERIMENT_TIMEOUT_SEC=600  # 10 minutes per experiment
readonly TSV_HEADER="iteration	timestamp	phase	commit_hash	fidelity_score	ssim	text_match	test_pass_rate	perf_score	status	description"

# ---------------------------------------------------------------------------
# Script directory and project root detection
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
PROJECT_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd -P)"
readonly SCRIPT_DIR PROJECT_ROOT

RESULTS_FILE="${SCRIPT_DIR}/results.tsv"
readonly RESULTS_FILE

# ---------------------------------------------------------------------------
# Global state (mutable)
# ---------------------------------------------------------------------------
CURRENT_ITERATION=0
CLEANUP_DONE=0
LOOP_STARTED=0
BEST_SCORE="0.000"

# ---------------------------------------------------------------------------
# Logging
# ---------------------------------------------------------------------------
_timestamp() {
    printf '%s' "$(date '+%Y-%m-%d %H:%M:%S')"
}

log_info() {
    printf '[%s] [INFO]  %s\n' "$(_timestamp)" "$*" >&2
}

log_warn() {
    printf '[%s] [WARN]  %s\n' "$(_timestamp)" "$*" >&2
}

log_error() {
    printf '[%s] [ERROR] %s\n' "$(_timestamp)" "$*" >&2
}

# ---------------------------------------------------------------------------
# Cleanup and signal handling
# ---------------------------------------------------------------------------
cleanup() {
    if (( CLEANUP_DONE == 1 )); then
        return 0
    fi
    CLEANUP_DONE=1

    # Skip cleanup if the experiment loop never started (e.g., --help, --version)
    if (( LOOP_STARTED == 0 )); then
        return 0
    fi

    log_info "Cleaning up... (iteration=${CURRENT_ITERATION})"

    # Revert any uncommitted changes to leave the tree clean
    if git -C "${PROJECT_ROOT}" diff --quiet 2>/dev/null; then
        log_info "Working tree is clean"
    else
        log_warn "Reverting uncommitted changes"
        git -C "${PROJECT_ROOT}" checkout -- . 2>/dev/null || true
    fi

    log_info "Experiment loop stopped after ${CURRENT_ITERATION} iterations"
    log_info "Best fidelity score: ${BEST_SCORE}"
    log_info "Results logged to: ${RESULTS_FILE}"
}

trap cleanup EXIT
trap 'log_info "Received SIGINT — stopping after current iteration"; exit 0' INT
trap 'log_info "Received SIGTERM — stopping after current iteration"; exit 0' TERM

# ---------------------------------------------------------------------------
# Usage and version
# ---------------------------------------------------------------------------
usage() {
    cat <<'USAGE'
Usage: run_loop.sh --phase <phase_name> [OPTIONS]

Required:
  --phase <name>          Phase program to run (e.g., 01_color_fidelity)

Options:
  --max-iterations <N>    Maximum iterations (default: 100)
  --help, -h              Show this help message
  --version               Show version

Examples:
  ./autoresearch/run_loop.sh --phase 01_color_fidelity
  ./autoresearch/run_loop.sh --phase 02_performance --max-iterations 50
USAGE
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
PHASE=""
MAX_ITERATIONS=100

parse_args() {
    if (( $# == 0 )); then
        usage
        exit 2
    fi

    while (( $# > 0 )); do
        case "$1" in
            --phase)
                if (( $# < 2 )); then
                    log_error "--phase requires a value"
                    exit 2
                fi
                PHASE="$2"
                shift 2
                ;;
            --max-iterations)
                if (( $# < 2 )); then
                    log_error "--max-iterations requires a value"
                    exit 2
                fi
                if ! [[ "$2" =~ ^[0-9]+$ ]]; then
                    log_error "--max-iterations must be a positive integer, got: $2"
                    exit 2
                fi
                MAX_ITERATIONS="$2"
                shift 2
                ;;
            --help|-h)
                usage
                exit 0
                ;;
            --version)
                printf 'run_loop.sh %s\n' "${VERSION}"
                exit 0
                ;;
            --)
                shift
                break
                ;;
            -*)
                log_error "Unknown option: $1"
                usage
                exit 2
                ;;
            *)
                log_error "Unexpected argument: $1"
                usage
                exit 2
                ;;
        esac
    done

    if [[ -z "${PHASE}" ]]; then
        log_error "--phase is required"
        usage
        exit 2
    fi
}

# ---------------------------------------------------------------------------
# Prerequisite checks
# ---------------------------------------------------------------------------
check_prerequisites() {
    local missing=0

    for cmd in cargo python3 git; do
        if ! command -v "${cmd}" &>/dev/null; then
            log_error "Required command not found: ${cmd}"
            missing=1
        fi
    done

    if (( missing == 1 )); then
        exit 3
    fi

    # Verify project root has Cargo.toml
    if [[ ! -f "${PROJECT_ROOT}/Cargo.toml" ]]; then
        log_error "Cargo.toml not found in project root: ${PROJECT_ROOT}"
        exit 4
    fi

    # Verify phase program exists
    local phase_file="${SCRIPT_DIR}/phases/${PHASE}.md"
    if [[ ! -f "${phase_file}" ]]; then
        log_error "Phase program not found: ${phase_file}"
        log_error "Available phases:"
        for f in "${SCRIPT_DIR}"/phases/*.md; do
            if [[ -f "${f}" ]]; then
                printf '  - %s\n' "$(basename "${f}" .md)" >&2
            fi
        done
        exit 2
    fi

    # Verify evaluate script exists
    if [[ ! -f "${PROJECT_ROOT}/evaluate/evaluate_fidelity.py" ]]; then
        log_error "Evaluation script not found: ${PROJECT_ROOT}/evaluate/evaluate_fidelity.py"
        exit 3
    fi

    # Verify results.tsv has correct header
    if [[ ! -f "${RESULTS_FILE}" ]]; then
        printf '%s\n' "${TSV_HEADER}" > "${RESULTS_FILE}"
        log_info "Created results.tsv with header"
    fi
}

# ---------------------------------------------------------------------------
# Core experiment steps
# ---------------------------------------------------------------------------

# Run cargo check with timeout
step_cargo_check() {
    log_info "[Step 1/4] cargo check"
    if ! timeout "${EXPERIMENT_TIMEOUT_SEC}s" cargo check \
            --manifest-path "${PROJECT_ROOT}/Cargo.toml" 2>&1; then
        return 1
    fi
    return 0
}

# Run cargo test with timeout
step_cargo_test() {
    log_info "[Step 2/4] cargo test"

    local test_output
    test_output="$(timeout "${EXPERIMENT_TIMEOUT_SEC}s" cargo test \
        --manifest-path "${PROJECT_ROOT}/Cargo.toml" 2>&1)" || true

    # Parse test results: "test result: ok. X passed; Y failed; Z ignored"
    local passed=0 failed=0 total=0
    if [[ "${test_output}" =~ ([0-9]+)\ passed ]]; then
        passed="${BASH_REMATCH[1]}"
    fi
    if [[ "${test_output}" =~ ([0-9]+)\ failed ]]; then
        failed="${BASH_REMATCH[1]}"
    fi
    total=$(( passed + failed ))

    if (( failed > 0 )); then
        log_error "Tests failed: ${failed}/${total}"
        printf '%s' "0.000"
        return 1
    fi

    # Calculate pass rate
    if (( total > 0 )); then
        # Bash integer arithmetic: multiply by 1000 first, then format
        local rate_x1000=$(( passed * 1000 / total ))
        printf '%d.%03d' $(( rate_x1000 / 1000 )) $(( rate_x1000 % 1000 ))
    else
        printf '1.000'
    fi
    return 0
}

# Run fidelity evaluation with timeout
step_evaluate() {
    log_info "[Step 3/4] evaluate fidelity"

    local eval_output
    eval_output="$(timeout "${EXPERIMENT_TIMEOUT_SEC}s" python3 \
        "${PROJECT_ROOT}/evaluate/evaluate_fidelity.py" \
        --project-root "${PROJECT_ROOT}" 2>&1)" || true

    # Parse FIDELITY_SCORE from output
    local fidelity="0.000" ssim="0.000" text_match="0.000"

    if [[ "${eval_output}" =~ FIDELITY_SCORE:\ *([0-9]+\.[0-9]+) ]]; then
        fidelity="${BASH_REMATCH[1]}"
    fi
    if [[ "${eval_output}" =~ SSIM:\ *([0-9]+\.[0-9]+) ]]; then
        ssim="${BASH_REMATCH[1]}"
    fi
    if [[ "${eval_output}" =~ TEXT_MATCH:\ *([0-9]+\.[0-9]+) ]]; then
        text_match="${BASH_REMATCH[1]}"
    fi

    printf '%s %s %s' "${fidelity}" "${ssim}" "${text_match}"
    return 0
}

# Compare two decimal scores: returns 0 if $1 > $2
score_improved() {
    local new="$1" old="$2"
    # Use awk for reliable floating-point comparison
    awk -v new="${new}" -v old="${old}" 'BEGIN { exit !(new > old) }'
}

# Append a row to results.tsv
log_result() {
    local iteration="$1" phase="$2" commit_hash="$3"
    local fidelity="$4" ssim="$5" text_match="$6"
    local test_pass_rate="$7" perf_score="$8"
    local status="$9" description="${10}"
    local ts
    ts="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
        "${iteration}" "${ts}" "${phase}" "${commit_hash}" \
        "${fidelity}" "${ssim}" "${text_match}" \
        "${test_pass_rate}" "${perf_score}" \
        "${status}" "${description}" >> "${RESULTS_FILE}"
}

# ---------------------------------------------------------------------------
# Main experiment loop
# ---------------------------------------------------------------------------
run_loop() {
    local phase="$1"
    local max_iter="$2"

    LOOP_STARTED=1

    log_info "=========================================="
    log_info "Autoresearch Experiment Loop"
    log_info "Phase:          ${phase}"
    log_info "Max iterations: ${max_iter}"
    log_info "Project root:   ${PROJECT_ROOT}"
    log_info "Results file:   ${RESULTS_FILE}"
    log_info "=========================================="

    # Get baseline score before starting
    log_info "Collecting baseline score..."
    local baseline_metrics
    baseline_metrics="$(step_evaluate)"
    read -r BEST_SCORE _ _ <<< "${baseline_metrics}"
    log_info "Baseline fidelity score: ${BEST_SCORE}"

    local iteration=0
    while (( iteration < max_iter )); do
        iteration=$(( iteration + 1 ))
        CURRENT_ITERATION="${iteration}"

        log_info "------------------------------------------"
        log_info "Iteration ${iteration}/${max_iter}"
        log_info "------------------------------------------"

        local status="revert"
        local commit_hash="-"
        local fidelity="0.000" ssim="0.000" text_match="0.000"
        local test_pass_rate="0.000" perf_score="-"
        local description="no changes detected"

        # Step 1: cargo check
        if ! step_cargo_check; then
            status="compile_fail"
            description="cargo check failed"
            log_warn "Compile failed — reverting"
            git -C "${PROJECT_ROOT}" checkout -- . 2>/dev/null || true
            log_result "${iteration}" "${phase}" "${commit_hash}" \
                "${fidelity}" "${ssim}" "${text_match}" \
                "${test_pass_rate}" "${perf_score}" \
                "${status}" "${description}"
            continue
        fi

        # Step 2: cargo test
        local test_result
        test_result="$(step_cargo_test)" || {
            status="test_fail"
            description="cargo test failed"
            log_warn "Tests failed — reverting"
            git -C "${PROJECT_ROOT}" checkout -- . 2>/dev/null || true
            log_result "${iteration}" "${phase}" "${commit_hash}" \
                "${fidelity}" "${ssim}" "${text_match}" \
                "${test_pass_rate}" "${perf_score}" \
                "${status}" "${description}"
            continue
        }
        test_pass_rate="${test_result}"

        # Step 3: evaluate fidelity
        local eval_metrics
        eval_metrics="$(step_evaluate)"
        read -r fidelity ssim text_match <<< "${eval_metrics}"

        log_info "Fidelity: ${fidelity} (best: ${BEST_SCORE}) | SSIM: ${ssim} | TextMatch: ${text_match}"

        # Step 4: decide keep or revert
        if score_improved "${fidelity}" "${BEST_SCORE}"; then
            # Check if there are actual changes to commit
            if ! git -C "${PROJECT_ROOT}" diff --quiet 2>/dev/null; then
                BEST_SCORE="${fidelity}"
                status="keep"
                description="score improved: ${BEST_SCORE}"

                git -C "${PROJECT_ROOT}" add -A
                git -C "${PROJECT_ROOT}" commit -m "$(cat <<EOF
experiment(${phase}): iteration ${iteration} — score ${fidelity}

Phase: ${phase}
Fidelity: ${fidelity} | SSIM: ${ssim} | TextMatch: ${text_match}
Test pass rate: ${test_pass_rate}
EOF
                )"

                commit_hash="$(git -C "${PROJECT_ROOT}" rev-parse --short HEAD)"
                log_info "KEEP — new best score: ${BEST_SCORE} (commit: ${commit_hash})"
            else
                status="revert"
                description="no code changes detected"
                log_info "No changes to commit"
            fi
        else
            status="revert"
            description="score did not improve (${fidelity} <= ${BEST_SCORE})"
            log_info "REVERT — score did not improve"
            git -C "${PROJECT_ROOT}" checkout -- . 2>/dev/null || true
        fi

        # Log result
        log_result "${iteration}" "${phase}" "${commit_hash}" \
            "${fidelity}" "${ssim}" "${text_match}" \
            "${test_pass_rate}" "${perf_score}" \
            "${status}" "${description}"
    done

    log_info "=========================================="
    log_info "Experiment loop completed"
    log_info "Total iterations: ${CURRENT_ITERATION}"
    log_info "Best fidelity score: ${BEST_SCORE}"
    log_info "=========================================="
}

# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------
main() {
    parse_args "$@"
    check_prerequisites
    run_loop "${PHASE}" "${MAX_ITERATIONS}"
}

main "$@"

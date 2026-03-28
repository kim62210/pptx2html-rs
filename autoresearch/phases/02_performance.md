# Phase: Rendering Performance Optimization

## Goal
Improve HTML rendering throughput (slides/sec) without sacrificing output quality.

## Editable Files (ONLY these)
- crates/pptx2html-core/src/renderer/mod.rs

## Read-Only Context
- crates/pptx2html-core/benches/pipeline.rs (benchmark reference)
- crates/pptx2html-core/src/model/* (data structures)
- evaluate/* (NEVER MODIFY)

## Hints
- String allocation is likely the main bottleneck — consider pre-allocation
- format!() creates a new String each time — consider write!() to shared buffer
- CSS class deduplication could reduce output size
- Check if any work is being repeated across slides

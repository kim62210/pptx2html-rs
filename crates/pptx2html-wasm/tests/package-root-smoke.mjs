import { execFile } from 'node:child_process';
import { mkdir, mkdtemp, symlink } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

const execFileAsync = promisify(execFile);

const packageDir = fileURLToPath(new URL('../pkg', import.meta.url));
const tempDir = await mkdtemp(path.join(tmpdir(), 'pptx2html-wasm-root-smoke-'));
const scopeDir = path.join(tempDir, 'node_modules', '@briank-dev');
const packageLink = path.join(scopeDir, 'pptx2html-turbo');

await mkdir(scopeDir, { recursive: true });
await symlink(packageDir, packageLink, 'dir');

const script = `
  import assert from 'node:assert/strict';
  import { readFile } from 'node:fs/promises';
  import init, {
    convert,
    convert_with_options_metadata,
    get_info,
  } from '@briank-dev/pptx2html-turbo';

  const wasmBytes = await readFile(new URL('./node_modules/@briank-dev/pptx2html-turbo/pptx2html_wasm_bg.wasm', import.meta.url));
  await init({ module_or_path: wasmBytes });

  assert.equal(typeof convert, 'function');
  assert.equal(typeof convert_with_options_metadata, 'function');
  assert.equal(typeof get_info, 'function');

  const invalidData = new Uint8Array([0, 1, 2, 3]);
  assert.throws(() => convert(invalidData), /invalid|zip|PPTX|archive/i);
  assert.throws(() => get_info(invalidData), /invalid|zip|PPTX|archive/i);
`;

await execFileAsync('node', ['--input-type=module', '--eval', script], {
  cwd: tempDir,
});

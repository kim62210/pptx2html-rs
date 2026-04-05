import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

import init, {
  convert,
  convert_slides,
  convert_with_metadata,
  convert_with_options,
  get_info,
  get_presentation_info,
  get_slide_count,
} from '../pkg/pptx2html_wasm.js';

const wasmBytes = await readFile(new URL('../pkg/pptx2html_wasm_bg.wasm', import.meta.url));
await init({ module_or_path: wasmBytes });

assert.equal(typeof convert, 'function');
assert.equal(typeof convert_slides, 'function');
assert.equal(typeof convert_with_options, 'function');
assert.equal(typeof convert_with_metadata, 'function');
assert.equal(typeof get_info, 'function');
assert.equal(typeof get_presentation_info, 'function');
assert.equal(typeof get_slide_count, 'function');

const invalidData = new Uint8Array([0, 1, 2, 3]);

assert.throws(() => convert(invalidData), /invalid|zip|PPTX|archive/i);
assert.throws(() => get_info(invalidData), /invalid|zip|PPTX|archive/i);

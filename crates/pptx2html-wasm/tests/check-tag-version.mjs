import assert from 'node:assert/strict';

const tagName = process.argv[2];
const expectedVersion = process.argv[3];

if (!tagName || !expectedVersion) {
  throw new Error('tag name and expected version arguments are required');
}

assert.match(tagName, /^v.+$/);
assert.equal(tagName.slice(1), expectedVersion);

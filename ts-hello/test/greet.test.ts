import test from "node:test";
import assert from "node:assert/strict";

import { greet } from "../src/greet.js";

test("greet preserves input casing", () => {
  assert.equal(greet("world"), "Hello, world!");
});

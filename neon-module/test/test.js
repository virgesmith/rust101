'use strict';
const assert = require('assert');
const nm = require("../native");

var data = { id: "me", values: [1, 2, 3], x: 1.0, sub: { id: "sub", flag: true } };

const res = nm.hello();
assert.equal(res.id, "node");
assert.deepEqual(res.values, [2,3,5,7,11,13,17,19]);

data = nm.objop(data);
assert.equal(data.id, "me.rs");
assert.equal(data.sub.id, "sub");

assert.strictEqual(nm.fibonacci(13), '233');

assert.strictEqual(nm.fibonacci(-13), "argument cannot be negative");
// error thrown but not caught 
// try {
//   nm.fibonacci(-13);
// }
// catch(e)
// {
//   console.log(e);
// }

//nm.fibonacci_async(-13, (e,r) => { if (e) console.error(e.message); else console.log(r); });

nm.fibonacci_async(13, (e,r) => { assert.equal(r, '233'); assert(e == null) });
nm.fibonacci_async(-13, (e,r) => { assert.equal(r, null); assert.equal(e.message, "argument cannot be negative") });
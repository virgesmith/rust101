'use strict';

const nm = require("../native");

var data = { id: "me", values: [1, 2, 3], x: 1.0, sub: { id: "sub", flag: true } };

console.log(nm.hello());

console.log(nm.cpu_count());

console.log(nm.objop(data));

console.log(nm.fibonacci(13));

console.log(nm.fibonacci(-13));
// error thrown but not caught 
// try {
//   nm.fibonacci(-13);
// }
// catch(e)
// {
//   console.log(e);
// }

nm.fibonacci_async(-13, (e,r) => { if (e) console.error(e); else console.log(r); });
'use strict';

const nm = require("../native");

var data = { id: "me", values: [1, 2, 3], x: 1.0, sub: { id: "sub", flag: true } };

console.log(nm.hello());

console.log(nm.cpu_count());

console.log(nm.objop(data));
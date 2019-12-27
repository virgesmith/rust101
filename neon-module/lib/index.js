'use strict';

const nm = require('../native');

module.exports.cpus = nm.cpu_count;
module.exports.hello = nm.hello;
module.exports.objop = nm.objop;

// alternatively load the native module directly
// neon_module=require("./native/")
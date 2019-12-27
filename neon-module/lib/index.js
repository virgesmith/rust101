var addon = require('../native');

module.exports.cpus = addon.thread_count;
module.exports.hello = addon.hello;
module.exports.objop = addon.objop;

// alternatively load the native module directly
// neon_module=require("./native/")
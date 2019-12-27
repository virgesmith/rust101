
extern crate neon;
extern crate neon_serde;
extern crate serde_derive;
extern crate num_cpus;

use neon::prelude::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: String,
		values: Vec<u32>,
		x: f64
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
	let object = Data {
		id: String::from("node"),
		values: vec![2,3,5,7,11,13,17,19],
		x: num_cpus::get() as f64
	};
	let js_value = neon_serde::to_value(&mut cx, &object)?;
	Ok(js_value)
}

fn objop(mut cx: FunctionContext) -> JsResult<JsValue> {
	let arg = cx.argument::<JsValue>(0)?;
	let mut value: Data = neon_serde::from_value(&mut cx, arg)?;
	value.id += "_";
	value.values.append(&mut vec![9,8,7]);
	value.x *= 1.5;
	//println!("{:?}", value);
	let retval = neon_serde::to_value(&mut cx, &value)?;
	Ok(retval)
}

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
	Ok(cx.number(num_cpus::get() as f64))
}

register_module!(mut cx, { 
	cx.export_function("hello", hello)?; 
	cx.export_function("objop", objop)?; 
	cx.export_function("thread_count", thread_count)?;
	Ok(())
});

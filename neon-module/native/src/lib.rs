
extern crate neon;
extern crate neon_serde;
extern crate serde_derive;
extern crate num_cpus;

use neon::prelude::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
struct Inner {
		id: String,
		flag: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: String,
		values: Vec<u32>,
		x: f64,
		sub: Inner 
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
	let object = Data {
		id: String::from("node"),
		values: vec![2,3,5,7,11,13,17,19],
		x: num_cpus::get() as f64,
		sub: Inner {
			id: String::from("node"),
			flag: false
		}
	};
	let json = neon_serde::to_value(&mut cx, &object)?;
	Ok(json)
}

fn objop(mut cx: FunctionContext) -> JsResult<JsValue> {
	let json_in = cx.argument::<JsValue>(0)?;
	let mut value: Data = neon_serde::from_value(&mut cx, json_in)?;
	value.id += ".rs";
	value.values.append(&mut vec![9,8,7]);
	value.x *= 1.5;
	//println!("{:?}", value);
	let json_out = neon_serde::to_value(&mut cx, &value)?;
	Ok(json_out)
}

fn cpu_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
	Ok(cx.number(num_cpus::get() as f64))
}

register_module!(mut cx, { 
	cx.export_function("hello", hello)?; 
	cx.export_function("objop", objop)?; 
	cx.export_function("cpu_count", cpu_count)?;
	Ok(())
});

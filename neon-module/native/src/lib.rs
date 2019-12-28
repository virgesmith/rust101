extern crate neon;
extern crate neon_serde;
extern crate num_bigint;
extern crate num_cpus;
extern crate num_traits;
extern crate serde_derive;

use neon::prelude::*;
use neon::register_module;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_derive::*;
use std::mem::replace;

#[derive(Debug, Serialize, Deserialize)]
struct Inner {
	id: String,
	flag: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
	id: String,
	values: Vec<u32>,
	x: f64,
	sub: Inner,
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
	let object = Data {
		id: String::from("node"),
		values: vec![2, 3, 5, 7, 11, 13, 17, 19],
		x: num_cpus::get() as f64,
		sub: Inner {
			id: String::from("node"),
			flag: false,
		},
	};
	let json = neon_serde::to_value(&mut cx, &object)?;
	Ok(json)
}

fn objop(mut cx: FunctionContext) -> JsResult<JsValue> {
	let json_in = cx.argument::<JsValue>(0)?;
	let mut value: Data = neon_serde::from_value(&mut cx, json_in)?;
	value.id += ".rs";
	value.values.append(&mut vec![9, 8, 7]);
	value.x *= 1.5;
	//println!("{:?}", value);
	let json_out = neon_serde::to_value(&mut cx, &value)?;
	Ok(json_out)
}

fn cpu_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
	Ok(cx.number(num_cpus::get() as f64))
}

fn compute(n: usize) -> BigUint {
	let mut f0: BigUint = Zero::zero();
	let mut f1: BigUint = One::one();
	for _ in 0..n {
		let f2 = f0 + &f1;
		// This is a low cost way of swapping f0 with f1 and f1 with f2.
		f0 = replace(&mut f1, f2);
	}
	f0
}

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsString> {
	let n = cx.argument::<JsNumber>(0)?.value();
	match n {
		n if n >= 0.0 => {
			let big = compute(n as usize);
			Ok(cx.string(big.to_str_radix(10)))
		},
		//_ => cx.throw_range_error("argument cannot be negative") //not caught in js catch???
		_ => Ok(cx.string("argument cannot be negative"))
	}
}

struct FibonacciTask {
	argument: f64
}


impl Task for FibonacciTask {
	type Output = BigUint;
	type Error = ();
	type JsEvent = JsString;

	fn perform(&self) -> Result<BigUint, ()> {
		match self.argument {
			n if n >= 0.0 => Ok(compute(self.argument as usize)),
			_ => Err(())
		}
	}

	fn complete(self, mut cx: TaskContext, result: Result<BigUint, ()>) -> JsResult<JsString> {
		match result {
			Ok(v) => Ok(cx.string(v.to_str_radix(10))),
			Err(_) => cx.throw_range_error("argument cannot be negative")
		}
	}
}

fn fibonacci_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let n = cx.argument::<JsNumber>(0)?.value();
	let cb = cx.argument::<JsFunction>(1)?;

	let task = FibonacciTask { argument: n };
	task.schedule(cb);

	Ok(cx.undefined())
}

register_module!(mut module, {
	module.export_function("hello", hello)?;
	module.export_function("objop", objop)?;
	module.export_function("cpu_count", cpu_count)?;
	module.export_function("fibonacci", fibonacci)?;
	module.export_function("fibonacci_async", fibonacci_async)?;
	Ok(())
});

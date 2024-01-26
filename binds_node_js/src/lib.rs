use neon::prelude::*;
mod modules;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let result = modules::functions::greet();
    Ok(cx.string(result))
}

fn add_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let result = modules::functions::add(a, b);
    Ok(cx.number(result))
}

fn custom_greet_js(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?.value(&mut cx);
    let age = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let height = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let result = modules::functions::custom_greet(name, age, height);
    Ok(cx.string(result))
}

fn calculate_hypotenuse_js(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let result = modules::functions::calculate_hypotenuse(a, b);
    Ok(cx.number(result))
}



#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("addNumbers", add_numbers)?;
    cx.export_function("customGreet", custom_greet_js)?;
    cx.export_function("calculateHypotenuse", calculate_hypotenuse_js)?;
    Ok(())
}

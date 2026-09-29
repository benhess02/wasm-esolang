use interpreter::Interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(code: &str) -> Result<String, JsValue> {
    validate_brackets(code)?;
    let mut vm = Interpreter::new(|_| {});
    let mut state = vm.interpret(code);
    let mut steps = 0;

    while state.code_ptr < state.chars.len() {
        if steps == 1_000_000 {
            return Err(JsValue::from_str("Step limit exceeded"));
        }
        state.step();
        steps += 1;
    }

    Ok(vm.output.iter().map(|&b| char::from(b)).collect())
}

fn validate_brackets(code: &str) -> Result<(), JsValue> {
    let mut depth = 0;

    for ch in code.chars() {
        match ch {
            '[' => depth += 1,
            ']' if depth == 0 => return Err(JsValue::from_str("Unmatched ]")),
            ']' => depth -= 1,
            _ => {}
        }
    }

    if depth != 0 {
        return Err(JsValue::from_str("Unmatched ["));
    }
    Ok(())
}

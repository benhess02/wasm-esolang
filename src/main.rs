use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut state = interpreter.interpret("+[+]");

    while state.code_ptr < state.chars.len() {
        state.step();
        println!("Location: {}", state.code_ptr);
        println!("{:?}", &state.interpreter.memory[0..10]);
    }

    println!("{}", interpreter.mem_ptr);
}

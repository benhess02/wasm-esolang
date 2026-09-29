pub struct Interpreter {
    pub memory: Vec<u8>,
    pub mem_ptr: usize,
    pub print_fn: fn(u8),
}

pub struct CodeState<'a> {
    pub interpreter: &'a mut Interpreter,
    pub chars: Vec<char>,
    pub code_ptr: usize,
    pub stack: Vec<usize>,
}

impl<'a> CodeState<'a> {
    pub fn run(&mut self) {
        while self.code_ptr < self.chars.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let ch = self.chars[self.code_ptr];
        self.code_ptr += 1;
        match ch {
            '+' => {
                self.interpreter.memory[self.interpreter.mem_ptr] =
                    self.interpreter.memory[self.interpreter.mem_ptr].wrapping_add(1);
            }
            '-' => {
                self.interpreter.memory[self.interpreter.mem_ptr] =
                    self.interpreter.memory[self.interpreter.mem_ptr].wrapping_sub(1);
            }
            '>' => {
                if self.interpreter.mem_ptr == self.interpreter.memory.len() - 1 {
                    self.interpreter.mem_ptr = 0;
                } else {
                    self.interpreter.mem_ptr += 1;
                }
            }
            '<' => {
                if self.interpreter.mem_ptr == 0 {
                    self.interpreter.mem_ptr = self.interpreter.memory.len() - 1;
                } else {
                    self.interpreter.mem_ptr -= 1;
                }
            }
            '[' => {
                if self.interpreter.memory[self.interpreter.mem_ptr] == 0 {
                    let mut count = 1;
                    while count > 0 && self.code_ptr < self.chars.len() {
                        match self.chars[self.code_ptr] {
                            '[' => count += 1,
                            ']' => count -= 1,
                            _ => {}
                        }
                        self.code_ptr += 1;
                    }
                } else {
                    self.stack.push(self.code_ptr - 1);
                }
            }
            ']' => {
                let target = self.stack.pop().unwrap();
                if self.interpreter.memory[self.interpreter.mem_ptr] != 0 {
                    self.code_ptr = target;
                }
            }
            '.' => (self.interpreter.print_fn)(self.interpreter.memory[self.interpreter.mem_ptr]),
            _ => {}
        }
    }
}

impl Interpreter {
    pub fn new(print_fn: fn(u8)) -> Self {
        Self {
            memory: vec![0u8; 30000],
            mem_ptr: 0,
            print_fn,
        }
    }

    pub fn interpret(&mut self, code: &str) -> CodeState<'_> {
        CodeState {
            interpreter: self,
            chars: code.chars().collect(),
            code_ptr: 0,
            stack: Vec::new(),
        }
    }
}

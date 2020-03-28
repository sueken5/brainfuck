fn main() {
    let input = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-------------.<<+++++++++++++++.>.+++.------.--------.>+..";
    let mut machine = Machine::new(input);
    machine.run();

    println!("",);
}


#[derive(Copy, Clone)]
enum Instruction {
    PointerIncrement, // >
    PointerDecrement, // <
    ValueIncrement, // +
    ValueDecrement, // -
    Write, // .
    Read, // ,
    LoopStart, // [
    LoopEnd, // ]
    Unknown,
}

struct Machine {
    state: [u8; 256],
    position: usize,
    pointer: usize,
    instructions: Vec<Instruction>,
}

impl Machine {
    pub fn new(input: &str) -> Self {
       let mut instructions: Vec<Instruction> = Vec::new();
        for x in input.chars() {
            match x {
                '>' => instructions.push(Instruction::PointerIncrement),
                '<' => instructions.push(Instruction::PointerDecrement),
                '+' => instructions.push(Instruction::ValueIncrement),
                '-' => instructions.push(Instruction::ValueDecrement),
                '.' => instructions.push(Instruction::Write),
                ',' => instructions.push(Instruction::Read),
                '[' => instructions.push(Instruction::LoopStart),
                ']' => instructions.push(Instruction::LoopEnd),
                _ => instructions.push(Instruction::Unknown),
            }
        }

        Machine {
            state: [0;256],
            position: 0,
            pointer: 0,
            instructions,
        }
    }

    pub fn run(&mut self) {
        while self.position < self.instructions.len() {
            let instruction = self.instructions[self.position];
            self.exec(instruction);
            self.position += 1
        }
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::PointerIncrement => self.pointer += 1,
            Instruction::PointerDecrement => self.pointer -= 1,
            Instruction::ValueIncrement => self.state[self.pointer] += 1,
            Instruction::ValueDecrement => self.state[self.pointer] -= 1,
            Instruction::Write => print!("{}", self.state[self.pointer] as char),
            Instruction::Read => {
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .expect("read_line error");
                let value = buf.as_bytes()[0];
                self.state[self.pointer] = value
            },
            Instruction::LoopStart => {
                match self.state[self.pointer] {
                    0 => {
                        let mut c = 1;
                        while c > 0 {
                            self.position  += 1;
                           match self.instructions[self.position] {
                               Instruction::LoopStart => c += 1,
                               Instruction::LoopEnd => c -= 1,
                               _ => {},
                           }
                        }
                    },
                    _ => {},
                }
            },
            Instruction::LoopEnd => {
                match self.state[self.pointer] {
                    0 => {},
                    _ => {
                        let mut c = 1;
                        while c > 0 {
                            self.position  -= 1;
                            match self.instructions[self.position] {
                                Instruction::LoopStart => c -= 1,
                                Instruction::LoopEnd => c += 1,
                                _ => {},
                            }
                        }
                    },
                }
            },
            Instruction::Unknown => panic!("unknown instruction"),
        }
    }
}
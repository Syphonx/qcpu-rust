pub struct Assembler {}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    pub fn assemble(&mut self, input: &str, output: &str) {
        print!("input: {}, output: {}", input, output);
    }
}

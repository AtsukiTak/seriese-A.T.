use at_asm::assemble;
use std::io::{stdin, stdout};

pub fn main() {
    assemble(&mut stdin(), &mut stdout());
}

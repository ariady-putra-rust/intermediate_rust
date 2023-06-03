use std::io::Result;

mod lambda;
mod thread_and_move;

#[allow(dead_code)]
pub enum IntermediateRust {
    Nothing,
    Lambda,
    ThreadAndMove,
}

pub fn run(intermediate_rust: IntermediateRust) -> Result<()> {
    match intermediate_rust {
        IntermediateRust::Lambda => lambda::lambda(),
        IntermediateRust::ThreadAndMove => thread_and_move::main_thread(),
        _ => Ok(()),
    }
}

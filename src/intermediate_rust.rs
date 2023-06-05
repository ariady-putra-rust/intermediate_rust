use std::io::Result;

mod lambda;
mod smart_pointers;
mod thread_and_move;

#[allow(dead_code)]
pub enum IntermediateRust {
    Nothing,
    Lambda,
    ThreadAndMove,
    SmartPointers,
}

pub fn run(intermediate_rust: IntermediateRust) -> Result<()> {
    match intermediate_rust {
        IntermediateRust::Lambda => lambda::lambda(),
        IntermediateRust::ThreadAndMove => thread_and_move::main_thread(),
        IntermediateRust::SmartPointers => {
            smart_pointers::smart_pointer(smart_pointers::SmartPointer::RefCellT)
        }
        _ => Ok(()),
    }
}

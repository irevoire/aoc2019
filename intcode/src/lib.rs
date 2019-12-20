pub mod communication;
pub mod tape;
pub mod vm;

pub use communication::VmCom;
pub use tape::Tape;
pub use vm::Vm;

type Error = Box<dyn std::error::Error>;

pub fn tape_from_file(file: &str) -> Result<Tape, Error> {
    std::fs::read_to_string(file)?.parse()
}

pub fn run_from_file(file: &str) -> Result<VmCom, Error> {
    let tape = tape_from_file(file)?;
    Ok(Vm::run_from(tape))
}

pub fn run_from_tape(tape: Tape) -> VmCom {
    Vm::run_from(tape)
}

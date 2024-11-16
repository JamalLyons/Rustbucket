pub mod assembler;
pub mod vm;

// Re-export commonly used items
pub use assembler::Assembler;
pub use vm::{VMConfig, VMError, CPU};

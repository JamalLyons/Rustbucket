/// Enum representing the different opcodes the VM can execute.
/// Each opcode represents a specific instruction that the VM can process.
#[derive(Debug)]
pub enum Opcode
{
    // Register Operations
    /// Increment: Adds 1 to the value in the specified register
    /// Usage: Inc(reg)
    /// Example: Inc(0) increments register 0
    Inc(u8),

    /// Decrement: Subtracts 1 from the value in the specified register
    /// Usage: Dec(reg)
    /// Example: Dec(0) decrements register 0
    Dec(u8),

    /// Output: Prints the value from the specified register to stdout
    /// Usage: Out(reg)
    /// Example: Out(0) prints the value in register 0
    Out(u8),

    /// Move: Loads an immediate value into the specified register
    /// Usage: Mov(dst_reg, immediate_value)
    /// Example: Mov(0, 5) loads the value 5 into register 0
    Mov(u8, u8),

    // Stack Operations
    /// Push: Pushes the value from the specified register onto the stack
    /// Usage: Push(reg)
    /// Example: Push(0) pushes the value from register 0 onto the stack
    Push(u8),

    /// Pop: Pops a value from the stack into the specified register
    /// Usage: Pop(reg)
    /// Example: Pop(0) pops the top stack value into register 0
    Pop(u8),

    /// Call: Pushes the next instruction address onto the call stack and jumps to the specified address
    /// Usage: Call followed by address byte
    /// Example: [0x12, 0x20] calls subroutine at address 0x20
    Call,

    /// Return: Pops the top address from the call stack and jumps to it
    /// Usage: Ret
    /// Example: 0x13 returns from subroutine
    Ret,

    // Memory Operations
    /// Load: Loads a value from memory into the specified register
    /// Usage: Load(reg) followed by address byte
    /// Example: [0x20, 0x00, 0x50] loads value at address 0x50 into register 0
    Load(u8),

    /// Store: Stores the value from the specified register into memory
    /// Usage: Store(reg) followed by address byte
    /// Example: [0x21, 0x00, 0x50] stores value from register 0 to address 0x50
    Store(u8),

    /// Load Indexed: Loads a value from memory using base address plus index
    /// Usage: LdIdx(reg) followed by base address byte
    /// The index is taken from register 1
    /// Example: [0x22, 0x00, 0x50] loads value from (0x50 + r1) into register 0
    LdIdx(u8),

    /// Store Indexed: Stores a value to memory using base address plus index
    /// Usage: StIdx(reg) followed by base address byte
    /// The index is taken from register 1
    /// Example: [0x23, 0x00, 0x50] stores value from register 0 to (0x50 + r1)
    StIdx(u8),

    // Arithmetic Operations
    /// Add: Adds the value from src register to dst register
    /// Usage: Add(dst_reg, src_reg)
    /// Example: Add(0, 1) adds r1 to r0, storing result in r0
    Add(u8, u8),

    /// Subtract: Subtracts the value in src register from dst register
    /// Usage: Sub(dst_reg, src_reg)
    /// Example: Sub(0, 1) subtracts r1 from r0, storing result in r0
    Sub(u8, u8),

    /// Multiply: Multiplies dst register by src register
    /// Usage: Mul(dst_reg, src_reg)
    /// Example: Mul(0, 1) multiplies r0 by r1, storing result in r0
    Mul(u8, u8),

    /// Divide: Divides dst register by src register
    /// Usage: Div(dst_reg, src_reg)
    /// Example: Div(0, 1) divides r0 by r1, storing result in r0
    /// Note: Triggers DivisionByZero error if src register contains 0
    Div(u8, u8),

    // Control Flow Operations
    /// Jump: Unconditional jump to specified address
    /// Usage: Jmp followed by address byte
    /// Example: [0x40, 0x20] jumps to address 0x20
    Jmp,

    /// Jump if Equal: Jumps if the zero flag is set (last comparison was equal)
    /// Usage: Jeq followed by address byte
    /// Example: [0x41, 0x20] jumps to 0x20 if zero flag is set
    Jeq,

    /// Jump if Greater: Jumps if the greater flag is set (last comparison was greater)
    /// Usage: Jgt followed by address byte
    /// Example: [0x42, 0x20] jumps to 0x20 if greater flag is set
    Jgt,

    /// Compare: Compares two registers and sets flags
    /// Usage: Cmp(reg1, reg2)
    /// Sets zero flag if reg1 == reg2
    /// Sets greater flag if reg1 > reg2
    /// Example: Cmp(0, 1) compares r0 with r1
    Cmp(u8, u8),

    /// Halt: Stops the program execution
    /// Usage: Halt (0xFF)
    /// Example: 0xFF halts the program
    Halt,

    /// Unknown: Represents an invalid or unsupported opcode
    /// Contains the invalid opcode byte that was encountered
    Unknown(u8),
}

impl From<u8> for Opcode
{
    /// Converts a raw byte (u8) into an `Opcode` enum variant.
    ///
    /// # Arguments
    /// * `byte` - The raw opcode byte fetched from memory.
    ///
    /// # Returns
    /// * `Opcode` - The corresponding enum variant for the opcode.
    fn from(byte: u8) -> Self
    {
        match byte {
            0x01 => {
                // Instructions with register operands need to read next byte
                Opcode::Inc(0) // Placeholder - actual register will be read in fetch
            }
            0x02 => Opcode::Dec(0),
            0x03 => Opcode::Out(0),
            0x04 => Opcode::Mov(0, 0),
            0x10 => Opcode::Push(0),
            0x11 => Opcode::Pop(0),
            0x12 => Opcode::Call,
            0x13 => Opcode::Ret,
            0x20 => Opcode::Load(0),
            0x21 => Opcode::Store(0),
            0x22 => Opcode::LdIdx(0),
            0x23 => Opcode::StIdx(0),
            0x30 => Opcode::Add(0, 0),
            0x31 => Opcode::Sub(0, 0),
            0x32 => Opcode::Mul(0, 0),
            0x33 => Opcode::Div(0, 0),
            0x40 => Opcode::Jmp,
            0x41 => Opcode::Jeq,
            0x42 => Opcode::Jgt,
            0x43 => Opcode::Cmp(0, 0),
            0xFF => Opcode::Halt,
            _ => Opcode::Unknown(byte),
        }
    }
}
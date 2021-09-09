use crate::instruction::{Address, Register, Value};

/// The type of branch/jump that will be taken by the instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchType {
	/// The instruction will unconditionally jump to the target.
	Unconditional,
	/// The instruction will jump to the target, pushing the calling
	/// address to the stack.
	Call,
	/// The instruction will jump if the register equals the value.
	Equal { register: Register, value: Value },
	/// The instruction will jump if the two registers equal.
	EqualRegister {
		register_a: Register,
		register_b: Register,
	},
	/// The instruction will jump if the key is pressed down.
	KeyPressed { value: Value },
}

/// Where the branch will jump if the condition is true.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchTarget {
	/// The instruction will jump to the target address.
	Address(Address),
	/// The instruction will skip the next instruction.
	Skip,
}

/// A branching instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchInstruction {
	/// The type of branch/jump that will be taken by the instruction.
	pub branch_type: BranchType,
	/// Where the branch will jump if the condition is true.
	pub branch_target: BranchTarget,
	/// Whether our condition is inverted or not.
	pub inverted: bool,
}

impl BranchInstruction {
	/// Creates a new branch instruction.
	pub fn new(branch_type: BranchType, branch_target: BranchTarget, inverted: bool) -> Self {
		BranchInstruction {
			branch_type,
			branch_target,
			inverted,
		}
	}
}

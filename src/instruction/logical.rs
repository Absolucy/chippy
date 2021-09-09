use crate::{instruction::Register, vm::Vm};

/// The type of logical operation to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
	/// Logical AND.
	And,
	/// Logical OR.
	Or,
	/// Logical XOR.
	Xor,
}

/// A logical instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalInstruction {
	/// The logical operation to perform.
	pub op: LogicalOp,
	/// The first register, which will be used as the first operand
	/// of the operation, and the destination register.
	pub register_a: Register,
	/// The second register, which will be used as the second operand
	/// of the operation.
	pub register_b: Register,
}

impl LogicalInstruction {
	pub fn execute(self, vm: &mut Vm) {
		let register_a = self.register_a as usize;
		let register_b = self.register_b as usize;
		assert!(register_a < vm.registers.len() && register_b < vm.registers.len());
		let value_b = vm.registers[register_b];
		let value_a = &mut vm.registers[register_a];
		match self.op {
			LogicalOp::And => *value_a &= value_b,
			LogicalOp::Or => *value_a |= value_b,
			LogicalOp::Xor => *value_a ^= value_b,
		}
	}
}

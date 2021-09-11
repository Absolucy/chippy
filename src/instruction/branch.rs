use crate::{
	instruction::{Address, Register, Value},
	vm::{ProgramCounter, Vm},
};

/// The type of branch/jump that will be taken by the instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BranchType {
	/// The instruction will unconditionally jump to the target.
	Unconditional,
	/// The instruction will jump to the target, pushing the calling
	/// address to the stack.
	Call,
	/// The instruction will jump if the register equals the value.
	#[display(fmt = "V{:X} == 0x{:X}", register, value)]
	Equal { register: Register, value: Value },
	/// The instruction will jump if the two registers equal.
	#[display(fmt = "V{:X} == V{:X}", register_a, register_b)]
	EqualRegister {
		register_a: Register,
		register_b: Register,
	},
	#[display(fmt = "Key(V{:X})", register)]
	/// The instruction will jump if the key is pressed down.
	KeyPressed { register: Register },
}

/// Where the branch will jump if the condition is true.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BranchTarget {
	/// The instruction will jump to the target address.
	#[display(fmt = "Jump to 0x{:X}", _0)]
	Address(Address),
	/// The instruction will jump to the target address + offset.
	#[display(fmt = "Jump to 0x{:X} + V{:X}", _0, _1)]
	AddressOffset(Address, Register),
	/// The instruction will skip the next instruction.
	#[display(fmt = "Skip Next Instruction")]
	Skip,
}

/// A branching instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[display(fmt = "If {} Then {}", branch_type, branch_target)]
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

	/// Execute the branch instruction.
	pub fn execute(self, vm: &mut Vm) -> ProgramCounter {
		let should_branch = match self.branch_type {
			BranchType::Unconditional => true,
			BranchType::Call => {
				vm.stack.push(vm.program_counter);
				true
			}
			BranchType::Equal { register, value } => {
				let register = register as usize;
				assert!(register < vm.registers.len());
				vm.registers[register] == value
			}
			BranchType::EqualRegister {
				register_a,
				register_b,
			} => {
				let register_a = register_a as usize;
				let register_b = register_b as usize;
				assert!(register_a < vm.registers.len() && register_b < vm.registers.len());
				vm.registers[register_a] == vm.registers[register_b]
			}
			BranchType::KeyPressed { register } => {
				let register = register as usize;
				assert!(register < vm.keypad.len());
				vm.keypad[vm.registers[register] as usize]
			}
		} ^ self.inverted;
		if should_branch {
			match self.branch_target {
				BranchTarget::Address(address) => ProgramCounter::Jump(address),
				BranchTarget::AddressOffset(address, offset_register) => {
					let offset_register = offset_register as usize;
					assert!(offset_register < vm.registers.len());
					let offset = vm.registers[offset_register] as u16;
					ProgramCounter::Jump(address + offset)
				}
				BranchTarget::Skip => ProgramCounter::Skip,
			}
		} else {
			ProgramCounter::Next
		}
	}
}

use crate::{
	instruction::{Register, Value},
	vm::Vm,
};

/// The type of arthimetic operation to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum ArthimeticOp {
	/// Addition.
	Add,
	/// Subtraction.
	Sub,
	/// Shift left
	Shl,
	/// Shift right
	Shr,
}

/// The values used for arthimetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum ArthimeticValue {
	/// A single register.
	#[display(fmt = "V{:X}", _0)]
	Register(Register),
	/// A register and a value.
	#[display(fmt = "V{:X}, {}", _0, _1)]
	RegisterValue(Register, Value),
	#[display(fmt = "V{:X}, V{:X}", _0, _1)]
	/// A register and a register.
	RegisterRegister(Register, Register),
}

/// An arthimetic instruction, using 2 registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[display(
	fmt = "{}{} {}{}",
	"if *inverted { \"NOT \" } else { \"\" }",
	"op",
	"values",
	"if *carry_flag { \" (carrying)\" } else { \"\" }"
)]
pub struct ArthimeticInstruction {
	/// The type of arthimetic operation to perform.
	pub op: ArthimeticOp,
	/// The two registers/values used for the operation.
	pub values: ArthimeticValue,
	/// Whether the carry flag will be set on overflow or not.
	pub carry_flag: bool,
	/// Whether the order of operands is inverted or not.
	/// Used for 8XY7.
	pub inverted: bool,
}

impl ArthimeticInstruction {
	/// Execute a single-register arhimetic instructiln.
	fn execute_register(&self, vm: &mut Vm, register: Register) {
		let register = register as usize;
		assert!(register < vm.registers.len());
		let value = vm.registers[register];
		match self.op {
			ArthimeticOp::Shl => {
				vm.registers[0xF] = (value >> 7) & 1;
				vm.registers[register] <<= 1;
			}
			ArthimeticOp::Shr => {
				vm.registers[0xF] = value & 1;
				vm.registers[register] >>= 1;
			}
			_ => unreachable!(),
		}
	}

	/// Execute a register-register arthimetic instruction.
	fn execute_register_register(&self, vm: &mut Vm, register_a: Register, register_b: Register) {
		let register_a = register_a as usize;
		let register_b = register_b as usize;
		assert!(register_a < vm.registers.len() && register_b < vm.registers.len());
		let value_a = vm.registers[register_a];
		let value_b = vm.registers[register_b];
		match self.op {
			ArthimeticOp::Add => {
				let (result, overflow) = value_a.overflowing_add(value_b);
				vm.registers[register_a] = result;
				if self.carry_flag {
					vm.registers[0xF] = overflow as u8;
				}
			}
			ArthimeticOp::Sub => {
				let (result, overflow) = if self.inverted {
					value_b.overflowing_sub(value_a)
				} else {
					value_a.overflowing_sub(value_b)
				};
				vm.registers[register_a] = result;
				if self.carry_flag {
					vm.registers[0xF] = !overflow as u8;
				}
			}
			_ => unreachable!(),
		}
	}

	fn execute_register_value(&self, vm: &mut Vm, register: Register, value: Value) {
		let register = register as usize;
		assert!(register < vm.registers.len());
		let register_value = vm.registers[register];
		match self.op {
			ArthimeticOp::Add => {
				let (result, overflow) = register_value.overflowing_add(value);
				vm.registers[register] = result;
				if self.carry_flag {
					vm.registers[0xF] = overflow as u8;
				}
			}
			ArthimeticOp::Sub => {
				let (result, overflow) = if self.inverted {
					value.overflowing_sub(register_value)
				} else {
					register_value.overflowing_sub(value)
				};
				vm.registers[register] = result;
				if self.carry_flag {
					vm.registers[0xF] = !overflow as u8;
				}
			}
			_ => unreachable!(),
		}
	}

	/// Execute the instruction on the CHIP-8 VM.
	pub fn execute(self, vm: &mut Vm) {
		match self.values {
			ArthimeticValue::Register(register) => self.execute_register(vm, register),
			ArthimeticValue::RegisterValue(register, value) => {
				self.execute_register_value(vm, register, value);
			}
			ArthimeticValue::RegisterRegister(register_a, register_b) => {
				self.execute_register_register(vm, register_a, register_b);
			}
		}
	}
}

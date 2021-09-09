use crate::{
	instruction::{Address, Register, Value},
	vm::Vm,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The target to load the value into.
pub enum LoadTarget {
	/// Load a register.
	Register(Register),
	/// Load an address.
	Address(Address),
	/// Load a value.
	Value(Value),
	/// Load I.
	I,
	/// Load the specific font.
	Font(u8),
	/// Load the delay timer.
	DelayTimer,
	/// Load the sound timer.
	SoundTimer,
	/// Load the BCD representation of a register
	Bcd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadInstruction {
	pub from: LoadTarget,
	pub into: LoadTarget,
}

impl LoadInstruction {
	/// Execute the load instruction.
	pub fn execute(self, vm: &mut Vm) {
		let value = match self.from {
			LoadTarget::Register(register) => {
				let register = register as usize;
				assert!(register <= 0xF);
				vm.registers[register] as u16
			}
			LoadTarget::Address(address) => {
				let address = address as usize;
				assert!(address <= 0x1000);
				vm.memory[address] as u16
			}
			LoadTarget::Value(value) => value as u16,
			LoadTarget::I => vm.index_register,
			LoadTarget::Font(_) => todo!("TODO: Implement font loading"),
			LoadTarget::DelayTimer => vm.delay_timer as u16,
			LoadTarget::SoundTimer => vm.sound_timer as u16,
			LoadTarget::Bcd => unreachable!("cannot load from bcd representation"),
		};
		match self.into {
			LoadTarget::Register(register) => {
				let register = register as usize;
				assert!(register <= 0xF);
				vm.registers[register] = value as u8;
			}
			LoadTarget::Address(address) => {
				let address = address as usize;
				assert!(address <= 0x1000);
				vm.memory[address] = value as u8;
			}
			LoadTarget::Value(_) => unreachable!("cannot load into a value"),
			LoadTarget::I => {
				vm.index_register = value;
			}
			LoadTarget::Font(_) => unreachable!("cannot load into a font"),
			LoadTarget::DelayTimer => {
				vm.delay_timer = value as u8;
			}
			LoadTarget::SoundTimer => {
				vm.sound_timer = value as u8;
			}
			LoadTarget::Bcd => {
				todo!("TODO: Implement BCD")
			}
		}
	}
}

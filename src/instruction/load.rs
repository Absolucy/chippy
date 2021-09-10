use crate::{
	instruction::{Address, Register, Value},
	vm::Vm,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
/// The target to load the value into.
pub enum LoadTarget {
	/// Load a register.
	#[display(fmt = "V{:X}", _0)]
	Register(Register),
	/// Load an address.
	#[display(fmt = "Memory(0x{:X})", _0)]
	Address(Address),
	/// Load a value.
	#[display(fmt = "0x{:X}", _0)]
	Value(Value),
	/// Load I.
	#[display(fmt = "I")]
	I,
	/// Load the specific font.
	#[display(fmt = "Font({:X})", _0)]
	Font(u8),
	/// Load the delay timer.
	#[display(fmt = "Delay Timer")]
	DelayTimer,
	/// Load the sound timer.
	#[display(fmt = "Sound Timer")]
	SoundTimer,
	/// Load the BCD representation of a register
	#[display(fmt = "BCD")]
	Bcd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[display(fmt = "Load from {} into {}", from, into)]
pub struct LoadInstruction {
	pub from: LoadTarget,
	pub into: LoadTarget,
}

impl LoadInstruction {
	/// Execute the load instruction.
	pub fn execute(self, vm: &mut Vm) {
		match (self.from, self.into) {
			(LoadTarget::Address(address), LoadTarget::I) => {
				vm.index_register = address;
			}
			(LoadTarget::Register(from), LoadTarget::Register(into)) => {
				let from = from as usize;
				let into = into as usize;
				assert!(from < vm.registers.len() && into < vm.registers.len());
				vm.registers[into] = vm.registers[from];
			}
			(LoadTarget::Value(from), LoadTarget::Register(into)) => {
				let into = into as usize;
				assert!(into < vm.memory.len());
				vm.registers[into] = from;
			}
			(LoadTarget::DelayTimer, LoadTarget::Register(into)) => {
				let into = into as usize;
				assert!(into < vm.registers.len());
				vm.registers[into] = vm.delay_timer;
			}
			(LoadTarget::Register(from), LoadTarget::DelayTimer) => {
				let from = from as usize;
				assert!(from < vm.registers.len());
				vm.delay_timer = vm.registers[from];
			}
			(LoadTarget::SoundTimer, LoadTarget::Register(into)) => {
				let into = into as usize;
				assert!(into < vm.registers.len());
				vm.registers[into] = vm.sound_timer;
			}
			(LoadTarget::Register(from), LoadTarget::SoundTimer) => {
				let from = from as usize;
				assert!(from < vm.registers.len());
				vm.sound_timer = vm.registers[from];
			}
			(LoadTarget::Font(from), LoadTarget::I) => {
				let address = 0x50 + (from as u16 * 5);
				vm.index_register = address;
			}
			(LoadTarget::Register(from), LoadTarget::Bcd) => {
				let from = from as usize;
				assert!(from < vm.registers.len());
				let value = vm.registers[from];
				let index = vm.index_register as usize;
				let bcd = [value / 100, (value / 10) % 10, value % 10];
				vm.memory[index..index + 3].copy_from_slice(&bcd);
			}
			(LoadTarget::I, LoadTarget::Register(into)) => {
				let from = vm.index_register as usize;
				let into = into as usize;
				assert!(from < vm.memory.len() && into < vm.registers.len());
				let register_range = into..vm.registers.len();
				let memory_range = from..(from + register_range.len());
				vm.registers[register_range].copy_from_slice(&vm.memory[memory_range]);
			}
			(LoadTarget::Register(from), LoadTarget::I) => {
				let from = from as usize;
				let into = vm.index_register as usize;
				assert!(into < vm.memory.len() && from < vm.registers.len());
				let register_range = from..vm.registers.len();
				let memory_range = into..(into + register_range.len());
				vm.memory[memory_range.clone()].copy_from_slice(&vm.registers[register_range]);
				vm.invalidate_cache(memory_range);
			}
			_ => panic!(
				"invalid load instruction: {:?} => {:?}",
				self.from, self.into
			),
		}
	}
}

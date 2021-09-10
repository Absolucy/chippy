use crate::instruction::{draw, Address, Instruction};
use bitvec::{array::BitArray, BitArr};
use fnv::FnvHashMap;
use std::ops::RangeBounds;

/// The CHIP-8 virtual machine and interpreter.
pub struct Vm {
	/// The memory of the CHIP-8 virtual machine.
	pub memory: [u8; 4096],
	/// The cache of parsed instructions.
	pub instruction_cache: FnvHashMap<u16, Instruction>,
	/// The registers of the CHIP-8 virtual machine.
	pub registers: [u8; 16],
	/// The index register of the CHIP-8 virtual machine.
	pub index_register: u16,
	/// The program counter of the CHIP-8 virtual machine.
	pub program_counter: u16,
	/// The stack of the CHIP-8 virtual machine.
	pub stack: Vec<u16>,
	/// The stack pointer of the CHIP-8 virtual machine.
	pub stack_pointer: u16,
	/// The delay timer of the CHIP-8 virtual machine.
	pub delay_timer: u8,
	/// The sound timer of the CHIP-8 virtual machine.
	pub sound_timer: u8,
	/// The keypad of the CHIP-8 virtual machine.
	pub keypad: BitArr!(for 0xF),
	/// The display of the CHIP-8 virtual machine.
	pub display: BitArr!(for 64 * 32),
}

impl Vm {
	/// Creates a new CHIP-8 virtual machine.
	pub fn new() -> Self {
		Self::default()
	}

	/// Loads a CHIP-8 program into the virtual machine.
	pub fn load_program(&mut self, program: &[u8]) {
		// Ensure the program is not too large (0x1000 - 0x200)
		assert!(program.len() <= 0xe00);
		// Reserve enough memory for the program's instructions in the instruction cache.
		self.instruction_cache
			.reserve(program.len() / 2 - self.instruction_cache.capacity());
		// Copy the program to memory.
		self.memory[0x200..0x200 + program.len()].copy_from_slice(program);
	}

	/// Invalidate the instruction cache for a memory range.
	/// This allows for self-modifying code, as we invalidate the memory range
	/// wherever memory is written.
	pub fn invalidate_cache<R>(&mut self, memory_range: R)
	where
		R: RangeBounds<usize>,
	{
		self.instruction_cache
			.retain(|key, _| !memory_range.contains(&(*key as usize)));
	}

	pub fn execute(&mut self) {
		assert!(self.program_counter >= 0x200 && self.program_counter < 0x1000);
		// Fetch the instruction from the instruction cache, or parse it into the cache.
		let instruction = *self
			.instruction_cache
			.entry(self.program_counter)
			.or_insert_with(|| {
				let opcode = u16::from_be_bytes([
					self.memory[self.program_counter as usize],
					self.memory[self.program_counter as usize + 1],
				]);
				println!("opcode {:X} at 0x{:X}", opcode, self.program_counter);
				Instruction::parse(opcode)
			});
		let next_step = match instruction {
			Instruction::Sys => panic!("attempted to do system call"),
			Instruction::Clear => {
				draw::clear(self);
				ProgramCounter::Next
			}
			Instruction::Return => {
				let return_address = self.stack.pop().unwrap();
				ProgramCounter::Jump(return_address)
			}
			Instruction::Random(_, _) => todo!("Random"),
			Instruction::Draw(x, y, row) => {
				draw::draw(self, x, y, row);
				ProgramCounter::Next
			}
			Instruction::LoadKey(_) => todo!("LoadKey"),
			Instruction::AddI(_) => todo!("AddI"),
			Instruction::Load(load) => {
				load.execute(self);
				ProgramCounter::Next
			}
			Instruction::Branch(branch) => branch.execute(self),
			Instruction::Logical(logic) => {
				logic.execute(self);
				ProgramCounter::Next
			}
			Instruction::Arthimetic(arthimetic) => {
				arthimetic.execute(self);
				ProgramCounter::Next
			}
		};
		next_step.next(self);
	}
}

impl Default for Vm {
	fn default() -> Self {
		Vm {
			instruction_cache: FnvHashMap::default(),
			memory: [0; 4096],
			registers: [0; 16],
			index_register: 0,
			program_counter: 0x200,
			stack: Vec::with_capacity(16),
			stack_pointer: 0,
			delay_timer: 0,
			sound_timer: 0,
			keypad: BitArray::zeroed(),
			display: BitArray::zeroed(),
		}
	}
}

pub enum ProgramCounter {
	Next,
	Skip,
	Jump(Address),
}

impl ProgramCounter {
	pub fn next(self, vm: &mut Vm) {
		match self {
			ProgramCounter::Next => vm.program_counter += 2,
			ProgramCounter::Skip => vm.program_counter += 4,
			ProgramCounter::Jump(address) => vm.program_counter = address,
		}
	}
}

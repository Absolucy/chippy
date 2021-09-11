use crate::instruction::{draw, Address, Instruction};
use bitvec::{array::BitArray, BitArr};
use fnv::FnvHashMap;
use nanorand::Rng;
use std::{
	ops::RangeBounds,
	time::{Duration, Instant},
};

const FONT: [u8; 80] = [
	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
	/// The delay timer of the CHIP-8 virtual machine.
	pub delay_timer: u8,
	/// The sound timer of the CHIP-8 virtual machine.
	pub sound_timer: u8,
	/// The keypad of the CHIP-8 virtual machine.
	pub keypad: BitArr!(for 0xF),
	/// The display of the CHIP-8 virtual machine.
	pub display: BitArr!(for 64 * 32),
	/// Whether the CHIP-8 virtual machine is paused or not.
	pub paused: bool,
	/// The number of cycles that the CHIP-8 virtual machine has executed.
	pub cycles: usize,
	/// How long the last cycle took for the CHIP-8 virtual machine to execute.
	pub last_cycle_time: Duration,
	/// The average cycle time of the CHIP-8 virtual machine.
	pub average_cycle_time: Duration,
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
		if self.paused {
			return;
		}
		assert!(self.program_counter >= 0x200 && self.program_counter < 0x1000);
		let start = Instant::now();
		// Fetch the instruction from the instruction cache, or parse it into the cache.
		let instruction = *self
			.instruction_cache
			.entry(self.program_counter)
			.or_insert_with(|| {
				let opcode = u16::from_be_bytes([
					self.memory[self.program_counter as usize],
					self.memory[self.program_counter as usize + 1],
				]);
				Instruction::parse(opcode)
					.unwrap_or_else(|| panic!("invalid opcode: {:04X}", opcode))
			});
		let next_step = match instruction {
			Instruction::Sys => panic!("attempted to do system call"),
			Instruction::Clear => {
				draw::clear(self);
				ProgramCounter::Next
			}
			Instruction::Return => {
				let return_address = self.stack.pop().expect("empty stack");
				ProgramCounter::Jump(return_address + 2)
			}
			Instruction::Random(register, value) => {
				let register = register as usize;
				assert!(register < self.registers.len());
				self.registers[register] = nanorand::tls_rng().generate::<u8>() & value;
				ProgramCounter::Next
			}
			Instruction::Draw(x, y, row) => {
				draw::draw(self, x, y, row);
				ProgramCounter::Next
			}
			Instruction::LoadKey(register) => {
				let register = register as usize;
				assert!(register < self.registers.len());
				match crate::subsystem::key::get_key() {
					Some(key) => {
						self.registers[register] = key as u8;
						ProgramCounter::Next
					}
					None => ProgramCounter::Pause,
				}
			}
			Instruction::AddI(register) => {
				let register = register as usize;
				assert!(register < self.registers.len());
				self.index_register += self.registers[register] as u16;
				ProgramCounter::Next
			}
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
		self.last_cycle_time = start.elapsed();
		self.average_cycle_time = (self.average_cycle_time + self.last_cycle_time) / 2;
		next_step.next(self);
		self.cycles += 1;
	}
}

impl Default for Vm {
	fn default() -> Self {
		let mut memory = [0; 4096];
		memory[0x50..=0x9F].copy_from_slice(&FONT);
		Vm {
			instruction_cache: FnvHashMap::default(),
			memory,
			registers: [0; 16],
			index_register: 0,
			program_counter: 0x200,
			stack: Vec::with_capacity(16),
			delay_timer: 0,
			sound_timer: 0,
			keypad: BitArray::zeroed(),
			display: BitArray::zeroed(),
			paused: false,
			cycles: 0,
			last_cycle_time: Duration::new(0, 0),
			average_cycle_time: Duration::new(0, 0),
		}
	}
}

pub enum ProgramCounter {
	Pause,
	Next,
	Skip,
	Jump(Address),
}

impl ProgramCounter {
	pub fn next(self, vm: &mut Vm) {
		match self {
			ProgramCounter::Pause => {}
			ProgramCounter::Next => vm.program_counter += 2,
			ProgramCounter::Skip => vm.program_counter += 4,
			ProgramCounter::Jump(address) => vm.program_counter = address,
		}
	}
}

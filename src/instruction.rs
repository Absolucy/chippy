pub mod arthimetic;
pub mod branch;
pub mod draw;
pub mod load;
pub mod logical;

use crate::vm::VmMode;

/// A CHIP-8 memory address.
pub type Address = u16;
/// A CHIP-8 register.
pub type Register = u8;
/// A CHIP-8 value.
pub type Value = u8;

/// A single CHIP-8 instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
	/// 0nnn - SYS addr
	/// This is a stub that panics.
	Sys,
	/// 00E0 - CLEAR
	Clear,
	/// 00EE - RETURN
	Return,
	/// Cxkk - RND Vx, byte
	#[display(fmt = "Random into V{:X} & 0x{:X}", _0, _1)]
	Random(Register, Value),
	/// Dxyn - DRW Vx, Vy, nibble
	#[display(fmt = "Draw Sprite at {},{} with {} rows", _0, _1, _2)]
	Draw(Register, Register, Value),
	/// Fx0A - LD Vx, K
	#[display(fmt = "Load Key into V{:X}", _0)]
	LoadKey(Register),
	/// Fx1E - ADD I, Vx
	#[display(fmt = "Add V{:X} to I", _0)]
	AddI(Register),
	#[display(
		fmt = "{} High Resolution Mode",
		"if *_0 { \"Enable\" } else { \"Disable \" }"
	)]
	SetHighResolution(bool),
	/// A loading instruction
	Load(load::LoadInstruction),
	/// A branching instruction (00EE, 1NNN, 2NNN, 3XNN, 4XNN, 5XY0 and 9XY0)
	Branch(branch::BranchInstruction),
	/// A logical operation (8XY1, 8XY2, and 8XY3)
	Logical(logical::LogicalInstruction),
	/// An arthimetic operation (8XY4, 8XY5, 8XY7, 8XY6, and 8XYE)
	Arthimetic(arthimetic::ArthimeticInstruction),
}

impl Instruction {
	/// Parses a CHIP-8 opcode
	pub fn parse(opcode: u16, mode: VmMode) -> Option<Self> {
		match opcode & 0xF000 {
			0x0000 => match opcode & 0x00FF {
				// 00E0 - CLS
				0x00E0 => Some(Instruction::Clear),
				// 00EE - RET
				0x00EE => Some(Instruction::Return),
				// 00FE - Disable High Resolution Mode
				0x00FE if mode == VmMode::SuperChip => Some(Instruction::SetHighResolution(false)),
				// 00FF - Enable High Resolution Mode
				0x00FF if mode == VmMode::SuperChip => Some(Instruction::SetHighResolution(true)),
				// 0nnn - SYS addr
				_ => Some(Instruction::Sys),
			},
			// 1nnn - JP addr
			0x1000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::Unconditional,
				branch_target: branch::BranchTarget::Address(opcode & 0x0FFF),
				inverted: false,
			})),
			// 2nnn - CALL addr
			0x2000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::Call,
				branch_target: branch::BranchTarget::Address(opcode & 0x0FFF),
				inverted: false,
			})),
			// 3xnn - SE Vx, byte
			0x3000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::Equal {
					register: ((opcode & 0x0F00) >> 8) as Register,
					value: (opcode & 0x00FF) as Value,
				},
				branch_target: branch::BranchTarget::Skip,
				inverted: false,
			})),
			// 4xkk - SNE Vx, byte
			0x4000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::Equal {
					register: ((opcode & 0x0F00) >> 8) as Register,
					value: (opcode & 0x00FF) as Value,
				},
				branch_target: branch::BranchTarget::Skip,
				inverted: true,
			})),
			// 5xy0 - SE Vx, Vy
			0x5000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::EqualRegister {
					register_a: ((opcode & 0x0F00) >> 8) as Register,
					register_b: ((opcode & 0x00F0) >> 4) as Register,
				},
				branch_target: branch::BranchTarget::Skip,
				inverted: false,
			})),
			// 6xkk - LD Vx, byte
			0x6000 => Some(Instruction::Load(load::LoadInstruction {
				from: load::LoadTarget::Value((opcode & 0x00FF) as Value),
				into: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
			})),
			// 7xkk - ADD Vx, byte
			0x7000 => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
				op: arthimetic::ArthimeticOp::Add,
				values: arthimetic::ArthimeticValue::RegisterValue(
					((opcode & 0x0F00) >> 8) as Register,
					(opcode & 0x00FF) as Value,
				),
				carry_flag: false,
				inverted: false,
			})),
			0x8000 => match opcode & 0x000F {
				// 8xy0 - LD Vx, Vy
				0x0000 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					into: load::LoadTarget::Register(((opcode & 0x00F0) >> 4) as Register),
				})),
				// 8xy1 - OR Vx, Vy
				0x0001 => Some(Instruction::Logical(logical::LogicalInstruction {
					op: logical::LogicalOp::Or,
					register_a: ((opcode & 0x0F00) >> 8) as Register,
					register_b: ((opcode & 0x00F0) >> 4) as Register,
				})),
				// 8xy2 - AND Vx, Vy
				0x0002 => Some(Instruction::Logical(logical::LogicalInstruction {
					op: logical::LogicalOp::And,
					register_a: ((opcode & 0x0F00) >> 8) as Register,
					register_b: ((opcode & 0x00F0) >> 4) as Register,
				})),
				// 8xy3 - XOR Vx, Vy
				0x0003 => Some(Instruction::Logical(logical::LogicalInstruction {
					op: logical::LogicalOp::Xor,
					register_a: ((opcode & 0x0F00) >> 8) as Register,
					register_b: ((opcode & 0x00F0) >> 4) as Register,
				})),
				// 8xy4 - ADD Vx, Vy
				0x0004 => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
					op: arthimetic::ArthimeticOp::Add,
					values: arthimetic::ArthimeticValue::RegisterRegister(
						((opcode & 0x0F00) >> 8) as Register,
						((opcode & 0x00F0) >> 4) as Register,
					),
					carry_flag: true,
					inverted: false,
				})),
				// 8xy5 - SUB Vx, Vy
				0x0005 => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
					op: arthimetic::ArthimeticOp::Sub,
					values: arthimetic::ArthimeticValue::RegisterRegister(
						((opcode & 0x0F00) >> 8) as Register,
						((opcode & 0x00F0) >> 4) as Register,
					),
					carry_flag: true,
					inverted: false,
				})),
				// 8xy6 - SHR Vx {, Vy}
				0x0006 if mode == VmMode::Chip8 => {
					Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
						op: arthimetic::ArthimeticOp::ShrOld,
						values: arthimetic::ArthimeticValue::RegisterRegister(
							((opcode & 0x0F00) >> 8) as Register,
							((opcode & 0x00F0) >> 4) as Register,
						),
						carry_flag: false,
						inverted: false,
					}))
				}
				// 8xy6 - SHR Vx , Vy
				0x0006 => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
					op: arthimetic::ArthimeticOp::Shr,
					values: arthimetic::ArthimeticValue::Register(
						((opcode & 0x0F00) >> 8) as Register,
					),
					carry_flag: false,
					inverted: false,
				})),
				// 8xy7 - SUBN Vx, Vy
				0x0007 => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
					op: arthimetic::ArthimeticOp::Sub,
					values: arthimetic::ArthimeticValue::RegisterRegister(
						((opcode & 0x0F00) >> 8) as Register,
						((opcode & 0x00F0) >> 4) as Register,
					),
					carry_flag: true,
					inverted: true,
				})),
				// 8xyE - SHL Vx, Vy
				0x000E if mode == VmMode::Chip8 => {
					Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
						op: arthimetic::ArthimeticOp::ShlOld,
						values: arthimetic::ArthimeticValue::RegisterRegister(
							((opcode & 0x0F00) >> 8) as Register,
							((opcode & 0x00F0) >> 4) as Register,
						),
						carry_flag: false,
						inverted: false,
					}))
				}
				// 8xyE - SHL Vx
				0x000E => Some(Instruction::Arthimetic(arthimetic::ArthimeticInstruction {
					op: arthimetic::ArthimeticOp::Shl,
					values: arthimetic::ArthimeticValue::Register(
						((opcode & 0x0F00) >> 8) as Register,
					),
					carry_flag: false,
					inverted: false,
				})),
				_ => None,
			},
			// 9xy0 - SNE Vx, Vy
			0x9000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::EqualRegister {
					register_a: ((opcode & 0x0F00) >> 8) as Register,
					register_b: ((opcode & 0x00F0) >> 4) as Register,
				},
				branch_target: branch::BranchTarget::Skip,
				inverted: true,
			})),
			// Annn - LD I, addr
			0xA000 => Some(Instruction::Load(load::LoadInstruction {
				from: load::LoadTarget::Address(opcode & 0x0FFF),
				into: load::LoadTarget::I,
			})),
			// Bxnn - JP V0, addr (CHIP-48)
			0xB000 if mode != VmMode::Chip8 => {
				Some(Instruction::Branch(branch::BranchInstruction {
					branch_type: branch::BranchType::Unconditional,
					branch_target: branch::BranchTarget::AddressOffset(
						opcode & 0x0FFF,
						((opcode & 0x0F00) >> 8) as Register,
					),
					inverted: false,
				}))
			}
			// Bnnn - JP V0, addr
			0xB000 => Some(Instruction::Branch(branch::BranchInstruction {
				branch_type: branch::BranchType::Unconditional,
				branch_target: branch::BranchTarget::AddressOffset(opcode & 0x0FFF, 0),
				inverted: false,
			})),
			// Cxkk - RND Vx, byte
			0xC000 => Some(Instruction::Random(
				((opcode & 0x0F00) >> 8) as Register,
				(opcode & 0x00FF) as Value,
			)),
			// Dxyn - DRW Vx, Vy, nibble
			0xD000 => Some(Instruction::Draw(
				((opcode & 0x0F00) >> 8) as Register,
				((opcode & 0x00F0) >> 4) as Register,
				(opcode & 0x000F) as Value,
			)),
			0xE000 => match opcode & 0x00FF {
				// Ex9E - SKP Vx
				0x009E => Some(Instruction::Branch(branch::BranchInstruction {
					branch_type: branch::BranchType::KeyPressed {
						register: ((opcode & 0x0F00) >> 8) as Register,
					},
					branch_target: branch::BranchTarget::Skip,
					inverted: false,
				})),
				// ExA1 - SKNP Vx
				0x00A1 => Some(Instruction::Branch(branch::BranchInstruction {
					branch_type: branch::BranchType::KeyPressed {
						register: ((opcode & 0x0F00) >> 8) as Register,
					},
					branch_target: branch::BranchTarget::Skip,
					inverted: true,
				})),
				_ => None,
			},
			0xF000 => match opcode & 0x00FF {
				// Fx07 - LD Vx, DT
				0x0007 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::DelayTimer,
					into: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
				})),
				// Fx0A - LD Vx, K
				0x000A => Some(Instruction::LoadKey(((opcode & 0x0F00) >> 8) as Register)),
				// Fx15 - LD DT, Vx
				0x0015 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					into: load::LoadTarget::DelayTimer,
				})),
				// Fx18 - LD ST, Vx
				0x0018 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					into: load::LoadTarget::SoundTimer,
				})),
				// Fx1E - ADD I, Vx
				0x001E => Some(Instruction::AddI(((opcode & 0x0F00) >> 8) as Register)),
				// Fx29 - LD F, Vx
				0x0029 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Font(((opcode & 0x0F00) >> 8) as u8),
					into: load::LoadTarget::I,
				})),
				// Fx33 - LD B, Vx
				0x0033 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					into: load::LoadTarget::Bcd,
				})),
				// Fx55 - LD [I], Vx
				0x0055 => Some(Instruction::Load(load::LoadInstruction {
					from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					into: if mode != VmMode::Chip8 {
						load::LoadTarget::IChip48
					} else {
						load::LoadTarget::I
					},
				})),
				// Fx65 - LD Vx, [I]
				0x0065 => Some(Instruction::Load(load::LoadInstruction {
					from: if mode != VmMode::Chip8 {
						load::LoadTarget::IChip48
					} else {
						load::LoadTarget::I
					},
					into: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
				})),
				// FX75 - store V0..VX in RPL user flags
				0x0075 if mode == VmMode::SuperChip => {
					Some(Instruction::Load(load::LoadInstruction {
						from: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
						into: load::LoadTarget::Rpl,
					}))
				}
				// FX85 - load V0..VX from RPL user flags
				0x0085 if mode == VmMode::SuperChip => {
					Some(Instruction::Load(load::LoadInstruction {
						from: load::LoadTarget::Rpl,
						into: load::LoadTarget::Register(((opcode & 0x0F00) >> 8) as Register),
					}))
				}
				_ => None,
			},
			_ => None,
		}
	}
}

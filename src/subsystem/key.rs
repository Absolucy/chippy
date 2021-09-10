use crate::vm::Vm;
use macroquad::prelude::*;

static KEY_BINDINGS: [(usize, KeyCode); 16] = [
	(0x1, KeyCode::Key1),
	(0x2, KeyCode::Key2),
	(0x3, KeyCode::Key3),
	(0xC, KeyCode::Key4),
	(0x4, KeyCode::Q),
	(0x5, KeyCode::W),
	(0x6, KeyCode::E),
	(0xD, KeyCode::R),
	(0x7, KeyCode::A),
	(0x8, KeyCode::S),
	(0x9, KeyCode::D),
	(0xE, KeyCode::F),
	(0xA, KeyCode::Z),
	(0x0, KeyCode::X),
	(0xB, KeyCode::C),
	(0xF, KeyCode::V),
];

pub static KEY_CHARS: [(char, usize); 16] = [
	('1', 0x1),
	('2', 0x2),
	('3', 0x3),
	('4', 0xC),
	('q', 0x4),
	('w', 0x5),
	('e', 0x6),
	('r', 0xD),
	('a', 0x7),
	('s', 0x8),
	('d', 0x9),
	('f', 0xE),
	('z', 0xA),
	('x', 0x0),
	('c', 0xB),
	('v', 0xF),
];

pub fn get_key() -> Option<usize> {
	get_char_pressed()
		.and_then(|c| {
			KEY_CHARS
				.iter()
				.find(|(x, _)| *x == c.to_ascii_lowercase())
				.copied()
		})
		.map(|(_, i)| i)
}

pub fn handle(vm: &mut Vm) {
	for (index, key) in KEY_BINDINGS.iter() {
		vm.keypad.set(*index, is_key_down(*key));
	}
}

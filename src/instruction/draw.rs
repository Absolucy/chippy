use crate::{
	instruction::{Register, Value},
	vm::Vm,
};

pub fn clear(vm: &mut Vm) {
	vm.display.set_all(false);
}

pub fn draw(vm: &mut Vm, x_reg: Register, y_reg: Register, rows: Value) {
	let x_reg = x_reg as usize;
	let y_reg = y_reg as usize;
	assert!(x_reg < vm.registers.len() && y_reg < vm.registers.len());
	let memory_location = vm.index_register as usize;
	let x = vm.registers[x_reg] % 64;
	let y = vm.registers[y_reg] % 32;
	vm.registers[0xF] = 0;
	for row in 0..rows {
		let y = (y + row) as usize;
		if y >= 32 {
			break;
		}
		let pixel = vm.memory[memory_location + row as usize];
		for col in 0..8 {
			let x = (x + col) as usize;
			if x >= 64 {
				break;
			}
			let index = 64 * y + x;
			let bit = (pixel >> (7 - col)) & 1 != 0;
			let set_pixel = vm.display[index];
			if bit && set_pixel {
				vm.registers[0xF] = 1;
			}
			vm.display.set(index, bit ^ set_pixel)
		}
	}
}

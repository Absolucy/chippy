pub mod instruction;
pub mod vm;

use macroquad::prelude::*;

#[macroquad::main("CHIP-8 Emulator")]
async fn main() {
	let rom = std::fs::read("rom.ch8").unwrap();
	let mut vm = vm::Vm::new();
	vm.load_program(&rom);
	let x_scale = screen_height() / 64.0;
	let y_scale = screen_width() / 32.0;
	loop {
		vm.execute();
		clear_background(BLACK);
		for (idx, pixel) in vm.display.iter().enumerate() {
			let x = (idx % 64) as f32 * x_scale;
			let y = (idx / 64) as f32 * y_scale;
			draw_rectangle(x, y, x_scale, y_scale, if *pixel { WHITE } else { BLACK });
		}
		next_frame().await;
	}
}

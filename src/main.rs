pub mod instruction;
pub mod subsystem;
pub mod vm;

use macroquad::prelude::*;
use std::time::{Duration, Instant};

#[macroquad::main("CHIP-8 Emulator")]
async fn main() {
	let rom = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
	let mut vm = vm::Vm::new();
	vm.load_program(&rom);
	let x_scale = screen_width() / 64.0;
	let y_scale = screen_height() / 32.0;
	let mut last_time = Instant::now();
	loop {
		subsystem::key::handle(&mut vm);
		vm.execute();
		if last_time.elapsed() > Duration::from_millis(16) {
			vm.delay_timer = vm.delay_timer.saturating_sub(1);
			vm.sound_timer = vm.sound_timer.saturating_sub(1);
			last_time = Instant::now();
		}
		clear_background(BLACK);
		for (idx, pixel) in vm.display.iter().enumerate() {
			let x = (idx % 64) as f32 * x_scale;
			let y = (idx / 64) as f32 * y_scale;
			draw_rectangle(x, y, x_scale, y_scale, if *pixel { WHITE } else { BLACK });
		}
		next_frame().await;
	}
}

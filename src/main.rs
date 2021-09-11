#[macro_use]
extern crate derive_more;

pub mod debugger;
pub mod instruction;
pub mod subsystem;
pub mod ui;
pub mod vm;

use crate::vm::Vm;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

fn step(vm: &mut Vm, last_time: &mut Instant) {
	subsystem::key::handle(vm);
	vm.execute();
	if last_time.elapsed() > Duration::from_millis(16) {
		vm.delay_timer = vm.delay_timer.saturating_sub(1);
		vm.sound_timer = vm.sound_timer.saturating_sub(1);
		*last_time = Instant::now();
	}
}

#[macroquad::main("CHIP-8 Emulator")]
async fn main() {
	let rom = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
	let mut vm = Vm::new();
	vm.load_program(&rom);
	let mut last_time = Instant::now();
	let mut show_debugger = false;
	loop {
		step(&mut vm, &mut last_time);
		clear_background(BLACK);
		let drawing_area = ui::draw(&mut vm);
		let (x_scale, y_scale) = (drawing_area.width() / 64.0, drawing_area.height() / 32.0);
		let (left, top) = (drawing_area.left(), drawing_area.top());
		for (idx, pixel) in vm.display.iter().enumerate() {
			let x = (idx % 64) as f32 * x_scale;
			let y = (idx / 64) as f32 * y_scale;
			draw_rectangle(
				left + x,
				top + y,
				x_scale,
				y_scale,
				if *pixel { WHITE } else { BLACK },
			);
		}
		if is_key_pressed(KeyCode::Period) {
			show_debugger = !show_debugger;
		}
		if is_key_pressed(KeyCode::Comma) {
			vm.paused = !vm.paused;
		}
		next_frame().await;
	}
}

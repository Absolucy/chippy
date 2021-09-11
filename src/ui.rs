mod cpu;
mod keypad;
mod memory;
mod menu;

use crate::vm::Vm;
use egui::{
	containers::panel::{CentralPanel, SidePanel, TopBottomPanel},
	Color32, Frame, Rect,
};

pub fn draw(vm: &mut Vm) -> Rect {
	let mut rect = Rect::NOTHING;
	egui_macroquad::ui(|ctx| {
		SidePanel::left("left").show(ctx, |ui| {
			cpu::draw(ui, vm);
		});
		SidePanel::right("right").show(ctx, |ui| {
			ui.label("Hello World!");
		});
		TopBottomPanel::bottom("bottom").show(ctx, |ui| {
			keypad::draw(ui, vm);
		});
		TopBottomPanel::top("menu").show(ctx, |ui| {
			menu::draw(ui, vm);
		});
		let central = CentralPanel::default()
			.frame(Frame::default().fill(Color32::TRANSPARENT))
			.show(ctx, |_| {});
		rect = central.response.rect;
	});
	egui_macroquad::draw();
	rect
}

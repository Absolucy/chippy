use crate::vm::Vm;
use egui::{Button, Grid, TextStyle, Ui};

fn key(key: usize, ui: &mut Ui, vm: &mut Vm) {
	vm.keypad.set(
		key,
		ui.add(Button::new(format!("{:X}", key)).text_style(TextStyle::Monospace))
			.clicked(),
	);
}

pub fn draw(ui: &mut Ui, vm: &mut Vm) {
	Grid::new("keypad").show(ui, |ui| {
		key(0x1, ui, vm);
		key(0x2, ui, vm);
		key(0x3, ui, vm);
		key(0xC, ui, vm);
		ui.end_row();

		key(0x4, ui, vm);
		key(0x5, ui, vm);
		key(0x6, ui, vm);
		key(0xD, ui, vm);
		ui.end_row();

		key(0x7, ui, vm);
		key(0x8, ui, vm);
		key(0x9, ui, vm);
		key(0xE, ui, vm);
		ui.end_row();

		key(0xA, ui, vm);
		key(0x0, ui, vm);
		key(0xB, ui, vm);
		key(0xF, ui, vm);
	});
}

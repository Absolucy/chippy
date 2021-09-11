use crate::vm::{Vm, VmMode};
use egui::{menu, Ui};
use rfd::FileDialog;

pub fn draw(ui: &mut Ui, vm: &mut Vm) {
	menu::bar(ui, |ui| {
		menu::menu(ui, "File", |ui| {
			if ui.button("Open").clicked() {
				if let Some(file) = FileDialog::new()
					.pick_file()
					.and_then(|file| std::fs::read(file).ok())
				{
					vm.load_program(&file);
				}
			}
			ui.selectable_value(&mut vm.mode, VmMode::Chip8, "CHIP-8");
			ui.selectable_value(&mut vm.mode, VmMode::Chip48, "CHIP-48");
			ui.selectable_value(&mut vm.mode, VmMode::SuperChip, "SUPER-CHIP");
		});
	});
}

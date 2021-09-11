use crate::vm::Vm;
use egui::{containers::CollapsingHeader, Grid, Label, Ui};

fn register(ui: &mut Ui, name: impl ToString, value: u16) {
	ui.vertical_centered_justified(|ui| {
		ui.add(Label::new(name).strong());
		ui.separator();
		ui.add(Label::new(format!("0x{:X}", value)).monospace());
	});
}

pub fn draw(ui: &mut Ui, vm: &mut Vm) {
	CollapsingHeader::new("CPU").show(ui, |ui| {
		Grid::new("cpu info").num_columns(2).show(ui, |ui| {
			ui.add(Label::new("Cycle Count").strong());
			ui.add(Label::new(format!("{}", vm.cycles)).monospace());
			ui.end_row();
			ui.add(Label::new("Cycle Length").strong());
			ui.add(Label::new(format!("{:?}", vm.last_cycle_time)).monospace());
			ui.end_row();
		});
		ui.separator();

		ui.horizontal(|ui| {
			ui.centered_and_justified(|ui| {
				ui.add(Label::new("Registers").strong());
				Grid::new("registers")
					.num_columns(4)
					.striped(true)
					.show(ui, |ui| {
						register(ui, "V0", vm.registers[0x0] as u16);
						register(ui, "V1", vm.registers[0x1] as u16);
						register(ui, "V2", vm.registers[0x2] as u16);
						register(ui, "V3", vm.registers[0x3] as u16);
						ui.end_row();

						register(ui, "V4", vm.registers[0x4] as u16);
						register(ui, "V5", vm.registers[0x5] as u16);
						register(ui, "V6", vm.registers[0x6] as u16);
						register(ui, "V7", vm.registers[0x7] as u16);
						ui.end_row();

						register(ui, "V8", vm.registers[0x8] as u16);
						register(ui, "V9", vm.registers[0x9] as u16);
						register(ui, "VA", vm.registers[0xA] as u16);
						register(ui, "VB", vm.registers[0xB] as u16);
						ui.end_row();

						register(ui, "VC", vm.registers[0xC] as u16);
						register(ui, "VD", vm.registers[0xD] as u16);
						register(ui, "VE", vm.registers[0xE] as u16);
						register(ui, "VF", vm.registers[0xF] as u16);
						ui.end_row();

						register(ui, "I", vm.index_register);
						register(ui, "PC", vm.program_counter);
						ui.end_row();
					});
			});
		});
	});
}

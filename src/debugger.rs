use crate::vm::Vm;
use egui::{
	containers::{Frame, ScrollArea},
	Label, Window,
};

pub fn debugger(vm: &mut Vm, paused: &mut bool) {
	egui_macroquad::ui(|egui_ctx| {
		let mut frame = Frame::default();
		frame.fill[3] = 0xF0;
		Window::new("Debugger").frame(frame).show(egui_ctx, |ui| {
			if *paused && ui.button("Unpause").clicked() {
				*paused = false;
			} else if !*paused && ui.button("Pause").clicked() {
				*paused = true;
			}
			ui.label(format!(
				"Cached Instructions: {}",
				vm.instruction_cache.len()
			));
			ui.collapsing("Registers", |ui| {
				for v in 0x0..=0xF {
					ui.horizontal(|ui| {
						ui.add(Label::new(format!("V{:X}", v)).strong().monospace());
						if *paused {
							let mut x = format!("0x{:X}", vm.registers[v as usize]);
							ui.text_edit_singleline(&mut x);
							vm.registers[v as usize] =
								u8::from_str_radix(x.trim().trim_start_matches("0x").trim(), 16)
									.unwrap_or(vm.registers[v as usize]);
						} else {
							ui.add(
								Label::new(format!("0x{:X}", vm.registers[v as usize])).monospace(),
							);
						}
					});
				}
				ui.horizontal(|ui| {
					ui.add(Label::new("I ").strong().monospace());
					if *paused {
						let mut x = format!("0x{:X}", vm.index_register);
						ui.text_edit_singleline(&mut x);
						vm.index_register =
							u16::from_str_radix(x.trim().trim_start_matches("0x").trim(), 16)
								.unwrap_or(vm.index_register);
					} else {
						ui.add(Label::new(format!("0x{:X}", vm.index_register)).monospace());
					}
				});
				ui.horizontal(|ui| {
					ui.add(Label::new("PC").strong().monospace());
					if *paused {
						let mut x = format!("0x{:X}", vm.program_counter);
						ui.text_edit_singleline(&mut x);
						vm.index_register =
							u16::from_str_radix(x.trim().trim_start_matches("0x").trim(), 16)
								.unwrap_or(vm.program_counter);
					} else {
						ui.add(Label::new(format!("0x{:X}", vm.program_counter)).monospace());
					}
				});
			});
			ui.collapsing("Stack", |ui| {
				for (idx, address) in vm.stack.iter().enumerate() {
					ui.horizontal(|ui| {
						ui.add(Label::new(format!("{:0<2} ", idx)).monospace());
						ui.add(Label::new(format!("0x{:X}", address)).monospace());
					});
				}
			});
			ui.collapsing("Instructions", |ui| {
				let row_height = ui.fonts()[egui::TextStyle::Body].row_height();
				let range = (0x200 / 2)..(vm.memory.len() / 2);
				ScrollArea::auto_sized().show_rows(ui, row_height, range.len(), |ui, row_range| {
					for row in row_range {
						let address = (0x200 + (row * 2)) as u16;
						match vm.instruction_cache.get(&address) {
							Some(instruction) => ui.horizontal(|ui| {
								ui.add(Label::new(format!("0x{:03X} ", address)).monospace());
								ui.add(Label::new(instruction.to_string()).monospace());
							}),
							None => ui.horizontal(|ui| {
								ui.add(Label::new(format!("0x{:03X} ", address)).monospace());
								ui.add(Label::new("???").monospace());
							}),
						};
					}
				})
			});
		});
	});
	egui_macroquad::draw();
}

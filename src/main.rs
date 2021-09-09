pub mod instruction;
pub mod vm;

fn main() {
	let rom = std::fs::read("rom.ch8").unwrap();
	let mut vm = vm::Vm::new();
	vm.load_program(&rom);
}

mod vm;

use std::{env};
use vm::Vm;

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let mut vm = Vm::default();

  vm.load_bin(file_path);
  vm.exec();
}

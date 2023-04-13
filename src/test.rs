#[warn(unused_must_use)]

#[cfg(test)]
mod tests {
  use std::fs;
  use crate::*;

  fn compile(filname: &str, output_name: &str) {
    match run(filname, output_name) {
      Ok(_) => {},
      Err(e) => {
        println!("Failed to compile: {e}");
      }
    };
  }

  #[test]
  fn it_assembles() {
    compile("test/basic_test.asm", "basic_test.bin");

    let result = match fs::read_to_string("basic_test.bin") {
      Err(_) => {
        String::new()
      },
      Ok(val) => {
        val
      }
    };

    let correct_result = match fs::read_to_string("test/basic_test.bin") {
      Err(e) => {
        println!("{e}");
        String::new()
      },
      Ok(val) => {
        println!("result: {val}");
        val
      }
    };

    assert_eq!(result, correct_result);

    fs::remove_file("basic_test.bin").expect("Unable to delete file");
  }

  #[test]
  fn it_loads_bin() {
    let mut vm = vm::Vm::new();
    vm.load_bin("test/basic_test.bin");

    assert_eq!(vm.instructions.len(), 3);
  }

  #[test]
  fn it_sets() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000010000011_u32); // SET $0 0 -3

    vm.exec();

    assert_eq!(vm.registers[0], -3);
  }

  #[test]
  fn it_adds() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000000000001_u32); // SET $0 0 1
    vm.instructions.push(0b001000000000000010_u32); // ADD $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 3);
  }

  #[test]
  fn it_prints() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000000000001_u32); // SET $0 0 1
    vm.instructions.push(0b000000001000000000_u32); // PRT $0 2 0

    vm.exec();

    assert_eq!(vm.output, String::from("output: 1"));
  }

  #[test]
  fn it_subtracts() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000000000001_u32); // SET $0 0 1
    vm.instructions.push(0b001100000000000010_u32); // SUB $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], -1);
  }

  #[test]
  fn it_multiplies() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000000000010_u32); // SET $0 0 2
    vm.instructions.push(0b010000000000000010_u32); // MUL $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 4);
  }

  #[test]
  fn it_divides() {
    let mut vm = vm::Vm::new();
    vm.instructions.push(0b000100000000000010_u32); // SET $0 0 2
    vm.instructions.push(0b010100000000000010_u32); // DIV $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 1);
  }

  #[test]
  fn it_jumps() {
    let mut vm = vm::Vm::new();

    compile("test/jump_test.asm", "jump_test.bin");
    vm.load_bin("jump_test.bin");
    vm.exec();

    assert_eq!(vm.registers[0], 11);

    fs::remove_file("jump_test.bin").expect("unable to remove");
  }

  #[test]
  fn it_supports_print_shortform() {
    let mut vm = vm::Vm::new();

    compile("test/print_test.asm", "print_test.bin");

    vm.load_bin("print_test.bin");
    vm.exec();

    assert_eq!(vm.output, "output: 12");

    fs::remove_file("print_test.bin").expect("unable to remove");
  }

  #[test]
  fn it_supports_set_shortform() {
    let mut vm = vm::Vm::new();
    compile("test/set_test.asm", "set_test.bin");
    vm.load_bin("set_test.bin");

    vm.exec();

    assert_eq!(vm.registers[0], 9);
    assert_eq!(vm.registers[1], 122);

    fs::remove_file("set_test.bin").expect("unable to remove");
  }

  #[test]
  fn it_does_labels() {
    let mut vm = vm::Vm::new();
    compile("test/label_test.asm", "label_test.bin");
    vm.load_bin("label_test.bin");

    vm.exec();

    assert_eq!(vm.output, "output: 5");

    fs::remove_file("label_test.bin").expect("unable to remove");
  }

  #[test]
  fn it_uses_extensions() {
    let mut vm = vm::Vm::new();
    compile("test/extension_test.asm", "extension_test.bin");
    vm.load_bin("extension_test.bin");

    vm.exec();

    assert_eq!(vm.registers[1], 3);
    assert_eq!(vm.registers[2], 5);
    assert_eq!(vm.registers[3], 2);
    assert_eq!(vm.registers[4], 8);
    assert_eq!(vm.registers[5], 1);

    fs::remove_file("extension_test.bin").expect("unable to remove");
  }
}

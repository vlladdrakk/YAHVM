#[cfg(test)]
mod tests {
  use std::fs;
  use crate::*;

  #[test]
  fn it_assembles() {
    run("example.asm");
    let result = match fs::read_to_string("out.bin") {
      Err(e) => {
        String::new()
      },
      Ok(val) => {
        String::from(val)
      }
    };

    let correct_result = match fs::read_to_string("test/out.bin") {
      Err(e) => {
        println!("{e}");
        String::new()
      },
      Ok(val) => {
        println!("result: {val}");
        String::from(val)
      }
    };

    assert_eq!(result, correct_result)
  }

  #[test]
  fn it_loads_bin() {
    let mut vm = vm::Vm::default();
    vm.load_bin("test/out.bin");

    assert_eq!(vm.instructions.len(), 3);
  }

  #[test]
  fn it_sets() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000010000011")); // SET $0 0 -3

    vm.exec();

    assert_eq!(vm.registers[0], -3);
  }

  #[test]
  fn it_adds() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000001")); // SET $0 0 1
    vm.instructions.push(String::from("001000000000000010")); // ADD $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 3);
  }

  #[test]
  fn it_prints() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000001")); // SET $0 0 1
    vm.instructions.push(String::from("000000001000000000")); // PRT $0 2 0

    vm.exec();

    assert_eq!(vm.output, String::from("output: 1"));
  }

  #[test]
  fn it_subtracts() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000001")); // SET $0 0 1
    vm.instructions.push(String::from("001100000000000010")); // SUB $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], -1);
  }

  #[test]
  fn it_multiplies() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000010")); // SET $0 0 2
    vm.instructions.push(String::from("010000000000000010")); // MUL $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 4);
  }

  #[test]
  fn it_divides() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000010")); // SET $0 0 2
    vm.instructions.push(String::from("010100000000000010")); // DIV $0 0 2

    vm.exec();

    assert_eq!(vm.registers[0], 1);
  }

  #[test]
  fn it_jumps() {
    let mut vm = vm::Vm::default();

    vm.set_tick_limit(15);

    vm.load_bin("test/jump_test.bin");
    vm.print_state();

    vm.exec();

    vm.print_state();

    assert_eq!(vm.registers[0], 0);
  }
}

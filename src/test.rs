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
    vm.load_bin("out.bin");

    assert_eq!(vm.instructions.len(), 3);
  }

  #[test]
  fn it_sets() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000001")); // SET $0 0 1

    vm.exec();

    assert_eq!(vm.registers[0], 1);
  }

  #[test]
  fn it_adds() {
    let mut vm = vm::Vm::default();
    vm.instructions.push(String::from("000100000000000001")); // SET $0 0 1
    vm.instructions.push(String::from("00100000000000001")); // ADD $0 0 2

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
}

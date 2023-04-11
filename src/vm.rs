mod var;

use std::{fs};
use var::Var;

pub struct Vm {
  pc: usize,
  pub registers: [Var; 18],
  pub instructions: Vec<String>,
  pub output: String,
  pub jump: bool,
  pub ticks: usize,
  tick_limit: usize,
}

impl Vm {
  pub fn default() -> Vm {
    let vm = Vm {
      pc: 0,
      registers: [Var::Integer(0); 18],
      instructions: Vec::<String>::new(),
      output: String::new(),
      jump: false,
      ticks: 0,
      tick_limit: usize::MAX
    };

    return vm
  }

  // pub fn set_tick_limit(&mut self, limit: usize) {
  //   self.tick_limit = limit;
  // }

  // pub fn print_state(&self) {
  //   println!("pc: {}", self.pc);
  //   println!("registers: {:?}", self.registers);
  //   println!("instructions: {:?}", self.instructions);
  //   println!("output: {}", self.output);
  //   println!("jump: {}", self.jump);
  //   println!("ticks: {}", self.ticks);
  // }

  pub fn load_bin(&mut self, filename: &str) {
    let data = match fs::read_to_string(filename) {
      Ok(d) => d,
      Err(e) => {
        panic!("An error occurd reading {filename}: {e}");
      }
    };
    self.instructions = data.trim().split('\n').map(|x| x.to_string()).collect();
  }

  pub fn exec(&mut self) {
    let mut instruction = self.fetch();

    while instruction != String::from("null") {
      let opcode = &instruction[0..4];

      match opcode {
        "0000" => self.prt(instruction),
        "0001" => self.set(instruction),
        "0010" => self.add(instruction),
        "0011" => self.sub(instruction),
        "0100" => self.mul(instruction),
        "0101" => self.div(instruction),
        "0110" => self.jmp(instruction),
        "0111" => self.jnp(instruction),
        "1000" => self.eql(instruction),
        "1001" => self.cbp(instruction),
        "1010" => self.clp(instruction),
        _ => {},
      }

      self.pc += 1;
      self.ticks += 1;

      if self.ticks > self.tick_limit {
        break;
      }

      instruction = self.fetch();
    }

  }

  // Returns the next instruction
  fn fetch(&self) -> String {
    if self.pc == self.instructions.len() {
      return String::from("null");
    }

    return String::from(&self.instructions[self.pc]);
  }

  fn instruction_value(&mut self, instruction: &str) -> Var {
    let ins_type = &instruction[8..10];

    match ins_type {
      "00" => Var::Integer(
        i8::from_str_radix(&instruction[11..18], 2).unwrap()
      ),
      "01" => Var::Float(
        // TODO: implement Float
        i8::from_str_radix(&instruction[11..18], 2).unwrap() as f32
      ),
      "10" => self.registers[
        i8::from_str_radix(&instruction[11..18], 2).unwrap() as usize
      ],
      _ => Var::Integer(0),
    }
  }

  fn prt(&mut self, instruction: String) {
    let ins_type = &instruction[8..10];

    match ins_type {
      "00" => {
        let num = i8::from_str_radix(&instruction[10..18], 2).unwrap();
        println!("output: {num}");
        self.output = String::from(format!("output: {num}"));
      },
      "10" => {
        let var = &instruction[4..8];
        let index = usize::from_str_radix(var, 2).unwrap();
        println!("output: {}", self.registers[index]);
        self.output = String::from(format!("output: {}", self.registers[index]));
      },
      _ => {}
    }
  }

  fn set(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let ins_type = &instruction[8..10];

    match ins_type {
      "00" => self.set_integer(index, instruction),
      "01" => self.set_float(index, instruction),
      "10" => self.set_with_variable(index, instruction),
      // "11" => "func_pointer",
      _ => (),
    };
  }

  fn set_integer(&mut self, index: usize, instruction: String) {
    let mut num = i8::from_str_radix(&instruction[11..18], 2).unwrap();
    let sign = instruction.chars().nth(10).unwrap();

    if sign == '1' {
      num = num * -1;
    }

    self.registers[index] = Var::Integer(num);
  }

  fn set_float(&mut self, index: usize, instruction: String) {
    // TODO: implement this
    let mut num = i8::from_str_radix(&instruction[11..18], 2).unwrap();
    let sign = instruction.chars().nth(10).unwrap();

    if sign == '1' {
      num = num * -1;
    }

    self.registers[index] = Var::Float(num as f32);
  }

  fn set_with_variable(&mut self, index: usize, instruction: String) {
    let source_var_str = &instruction[11..18];
    let source_var = usize::from_str_radix(source_var_str, 2).unwrap();

    self.registers[index] = self.registers[source_var];
  }

  fn add(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] = match self.registers[index] {
      Var::Integer(i) => Var::Integer(i + num),
      Var::Float(f) => Var::Float(f + num as f32),
    }
  }

  fn sub(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] = match self.registers[index] {
      Var::Integer(i) => Var::Integer(i - num),
      Var::Float(f) => Var::Float(f - num as f32),
    }
  }

  fn mul(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] = match self.registers[index] {
      Var::Integer(i) => Var::Integer(i * num),
      Var::Float(f) => Var::Float(f * num as f32),
    }
  }

  fn div(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] = match self.registers[index] {
      Var::Integer(i) => Var::Integer(i / num),
      Var::Float(f) => Var::Float(f / num as f32),
    }
  }

  fn jmp(&mut self, instruction: String) {
    if self.jump {
      let num = usize::from_str_radix(&instruction[11..18], 2).unwrap();
      if num < 2 {
        self.pc = 0;
      } else {
        self.pc = num - 2;
      }
    }
  }

  fn jnp(&mut self, instruction: String) {
    if !self.jump {
      let num = usize::from_str_radix(&instruction[11..18], 2).unwrap();
      self.pc = num - 2;
    }
  }

  fn eql(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let other_value = self.instruction_value(&instruction);

    self.jump = self.registers[index] == other_value;
  }

  fn cbp(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.jump = match self.registers[index] {
      Var::Integer(i) => i > num,
      Var::Float(f) => f > num as f32,
    }
  }

  fn clp(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.jump = match self.registers[index] {
      Var::Integer(i) => i < num,
      Var::Float(f) => f < num as f32,
    }
  }
}

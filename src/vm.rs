use std::fs;

pub struct Vm {
  pc: usize,
  pub registers: [i8; 18],
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
      registers: [0; 18],
      instructions: Vec::<String>::new(),
      output: String::new(),
      jump: false,
      ticks: 0,
      tick_limit: usize::MAX
    };

    return vm
  }

  pub fn set_tick_limit(&mut self, limit: usize) {
    self.tick_limit = limit;
  }

  pub fn print_state(&self) {
    println!("pc: {}", self.pc);
    println!("registers: {:?}", self.registers);
    println!("instructions: {:?}", self.instructions);
    println!("output: {}", self.output);
    println!("jump: {}", self.jump);
    println!("ticks: {}", self.ticks);
  }

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
    println!("pc: {}", self.pc);
    if self.pc == self.instructions.len() {
      return String::from("null");
    }

    return String::from(&self.instructions[self.pc]);
  }

  fn prt(&mut self, instruction: String) {
    let ins_type = &instruction[8..10];

    println!("type: {ins_type}");

    match ins_type {
      "00" => {
        let num = &instruction[10..18];
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

    let var_type = match ins_type {
      "00" => "int",
      "01" => "float",
      "10" => "var_pointer",
      "11" => "func_pointer",
      _ => ""
    };

    let mut num = i8::from_str_radix(&instruction[11..18], 2).unwrap();
    let sign = instruction.chars().nth(10).unwrap();

    if sign == '1' {
      num = num * -1;
    }

    self.registers[index] = num;
  }

  fn add(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] += num;
  }

  fn sub(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] -= num;
  }

  fn mul(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] *= num;
  }

  fn div(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.registers[index] /= num;
  }

  fn jmp(&mut self, instruction: String) {
    if self.jump {
      let num = usize::from_str_radix(&instruction[11..18], 2).unwrap();
      self.pc = num - 2;
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
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.jump = self.registers[index] == num;
  }

  fn cbp(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.jump = self.registers[index] > num;
  }

  fn clp(&mut self, instruction: String) {
    let var = &instruction[4..8];
    let index = usize::from_str_radix(var, 2).unwrap();
    let num = i8::from_str_radix(&instruction[11..18], 2).unwrap();

    self.jump = self.registers[index] < num;
  }
}
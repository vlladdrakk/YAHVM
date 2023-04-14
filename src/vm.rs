use std::{env, fs};

pub struct Vm {
  pc: usize,
  pub registers: [i8; 18],
  pub instructions: Vec<u32>,
  pub output: String,
  pub jump: bool,
  pub ticks: usize,
  tick_limit: usize,
}

const OPCODE_MASK: u32 = 0b111100000000000000;
const VAR_MASK: u32    = 0b000011110000000000;
const TYPE_MASK: u32   = 0b000000001100000000;
const NUM_MASK: u32    = 0b000000000011111111;

fn get_opcode(instruction: u32) -> u8 {
  ((instruction & OPCODE_MASK)  >> 14) as u8
}

#[test]
fn test_get_opcode() {
  assert_eq!(get_opcode(0b111100000000000000), 0b1111);
  assert_eq!(get_opcode(0b011000000000000000), 0b0110);
  assert_eq!(get_opcode(0b100100000000000000), 0b1001);
}

fn get_var(instruction: u32) -> u8 {
  ((instruction & VAR_MASK)  >> 10) as u8
}

#[test]
fn test_get_var() {
  assert_eq!(get_var(0b000011110000000000), 0b1111);
  assert_eq!(get_var(0b000001100000000000), 0b0110);
  assert_eq!(get_var(0b000010010000000000), 0b1001);
}

fn get_type(instruction: u32) -> u8 {
  ((instruction & TYPE_MASK)  >> 8) as u8
}

#[test]
fn test_get_type() {
  assert_eq!(get_type(0b000000000000000000), 0b00);
  assert_eq!(get_type(0b000000000100000000), 0b01);
  assert_eq!(get_type(0b000000001000000000), 0b10);
  assert_eq!(get_type(0b000000001100000000), 0b11);
}

fn get_num(instruction: u32) -> i8 {
  let mut unsigned_num = (instruction & NUM_MASK) as u8;

  if unsigned_num & 0b10000000 == 0b10000000 {
    unsigned_num &= 0b01111111;
    return -(unsigned_num as i8)
  }

  unsigned_num as i8
}

fn get_unum(instruction: u32) -> u8 {
  (instruction & NUM_MASK) as u8
}

#[test]
fn test_get_num() {
  assert_eq!(get_num(0b000000000000000001), 1);
  assert_eq!(get_num(0b000000000000000011), 3);
  assert_eq!(get_num(0b000000000000000100), 4);
  assert_eq!(get_num(0b000000000010000001), -1);
}

impl Default for Vm {
  fn default() -> Self {
    Self::new()
  }
}

impl Vm {
  pub fn new() -> Self {
    Self {
      pc: 0,
      registers: [0; 18],
      instructions: Vec::<u32>::new(),
      output: String::new(),
      jump: false,
      ticks: 0,
      tick_limit: usize::MAX
    }
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
    let data = match fs::read(filename) {
      Ok(d) => d,
      Err(e) => {
        panic!("An error occurd reading {filename}: {e}");
      }
    };

    // let mut index: usize = 0;
    let mut bytes:[u8; 4] = [0, 0, 0, 0];
    for (i, b) in data.iter().enumerate() {
      bytes[i % 4] = *b;

      if (i % 4 == 0 || i == data.len() - 1) && i > 0 {
        let ins: u32 = u32::from_be_bytes(bytes);
        self.instructions.push(ins);
        bytes = [0, 0, 0, 0];
      }
    }
  }

  pub fn exec(&mut self) {
    let mut instruction = match self.fetch() {
      Some(ins) => ins,
      None => return,
    };

    loop {
      let opcode: u8 = get_opcode(instruction);

      match opcode {
        0b0000 => self.prt(instruction),
        0b0001 => self.set(instruction),
        0b0010 => self.add(instruction),
        0b0011 => self.sub(instruction),
        0b0100 => self.mul(instruction),
        0b0101 => self.div(instruction),
        0b0110 => self.jmp(instruction),
        0b0111 => self.jnp(instruction),
        0b1000 => self.eql(instruction),
        0b1001 => self.cbp(instruction),
        0b1010 => self.clp(instruction),
        _ => {},
      }

      self.pc += 1;
      self.ticks += 1;

      if self.ticks > self.tick_limit {
        break;
      }

      instruction = match self.fetch() {
        Some(ins) => ins,
        None => return,
      };
    }

  }

  // Returns the next instruction
  fn fetch(&self) -> Option<u32> {
    if self.pc == self.instructions.len() {
      return None;
    }

    Some(self.instructions[self.pc])
  }

  fn prt(&mut self, instruction: u32) {
    let ins_type = ((instruction & TYPE_MASK) >> 8) as u8;

    match ins_type {
      0b00 => {
        let num = get_num(instruction);
        println!("output: {num}");
        self.output = format!("output: {num}");
      },
      0b10 => {
        let var = get_var(instruction);
        let index = var as usize;
        println!("output: {}", self.registers[index]);
        self.output = format!("output: {}", self.registers[index]);
      },
      _ => {}
    }
  }

  fn set(&mut self, instruction: u32) {
    let var = get_var(instruction);
    let index = var as usize;
    let ins_type = get_type(instruction);

    let _var_type = match ins_type {
      0b00 => "int",
      0b01 => "float",
      0b10 => "var_pointer",
      0b11 => "func_pointer",
      _ => ""
    };

    let num = get_num(instruction);

    if ins_type == 0b01 {
      if !(0..=15).contains(&num) {
        panic!("Register index out of bounds");
      }

      self.registers[index] = self.registers[num as usize];
      return;
    }

    self.registers[index] = num;
  }

  fn add(&mut self, instruction: u32) {
    let var = get_var(instruction);
    let index = var as usize;
    let num = get_num(instruction);
    let ins_type = get_type(instruction);

    if ins_type == 0b01 {
      if num > 15 {
        panic!("Register index out of bounds");
      }

      self.registers[index] += self.registers[num as usize];
      return;
    }

    self.registers[index] += num;
  }

  fn sub(&mut self, instruction: u32) {
    let var = get_var(instruction);
    let index = var as usize;
    let num = get_num(instruction);
    let ins_type = get_type(instruction);

    if ins_type == 0b01 {
      if num > 15 {
        panic!("Register index out of bounds");
      }

      self.registers[index] -= self.registers[num as usize];
      return;
    }

    self.registers[index] -= num;
  }

  fn mul(&mut self, instruction: u32) {
    let var = get_var(instruction);
    let index = var as usize;
    let num = get_num(instruction);
    let ins_type = get_type(instruction);

    if ins_type == 0b01 {
      if num > 15 {
        panic!("Register index out of bounds");
      }

      self.registers[index] *= self.registers[num as usize];
      return;
    }

    self.registers[index] *= num;
  }

  fn div(&mut self, instruction: u32) {
    let var = get_var(instruction);
    let index = var as usize;
    let num = get_num(instruction);
    let ins_type = get_type(instruction);

    if ins_type == 0b01 {
      if num > 15 {
        panic!("Register index out of bounds");
      }

      self.registers[index] /= self.registers[num as usize];
      return;
    }


    self.registers[index] /= num;
  }

  fn jmp(&mut self, instruction: u32) {
    if self.jump {
      let num = get_unum(instruction) as usize;
      if num < 2 {
        self.pc = 0;
      } else {
        println!("Jmp to {}", num - 2);
        self.pc = num - 2;
      }
    }
  }

  fn jnp(&mut self, instruction: u32) {
    if !self.jump {
      let num = get_unum(instruction) as usize;
      println!("JNp to {}", num - 2);
      if num < 2 {
        self.pc = 0;
      } else {
        self.pc = num - 2;
      }
    }
  }

  fn eql(&mut self, instruction: u32) {
    let index = get_var(instruction) as usize;
    let num = get_num(instruction);

    self.jump = self.registers[index] == num;
  }

  fn cbp(&mut self, instruction: u32) {
    let index = get_var(instruction) as usize;
    let num = get_num(instruction);

    self.jump = self.registers[index] > num;
  }

  fn clp(&mut self, instruction: u32) {
    let index = get_var(instruction) as usize;
    let num = get_num(instruction);

    self.jump = self.registers[index] < num;
  }
}

#[allow(dead_code)]
fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let mut vm = Vm::new();

  vm.load_bin(file_path);
  vm.exec();
}

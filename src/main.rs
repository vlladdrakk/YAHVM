mod test;
mod vm;

use std::{env, fs};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use phf::{phf_map};

static OPCODES : phf::Map<&'static str, &'static str> = phf_map! {
  "PRT" => "0000",
  "SET" => "0001",
  "ADD" => "0010",
  "SUB" => "0011",
  "MUL" => "0100",
  "DIV" => "0101",
  "JMP" => "0110",
  "JNP" => "0111",
  "EQL" => "1000",
  "CBP" => "1001",
  "CLP" => "1010",
};

static VARS : phf::Map<&'static str, &'static str> = phf_map! {
  "$0" => "0000",
  "$1" => "0001",
  "$2" => "0010",
  "$3" => "0011",
  "$4" => "0100",
  "$5" => "0101",
  "$6" => "0110",
  "$7" => "0111",
  "$8" => "1000",
  "$9" => "1001",
  "$a" => "10010",
  "$b" => "1011",
  "$c" => "1100",
  "$d" => "1101",
  "$e" => "1110",
  "$f" => "1111",
};

static TYPE : phf::Map<&'static str, &'static str> = phf_map! {
  "0" => "00",
  "1" => "01",
  "2" => "10",
  "3" => "11",
};

struct Instruction {
  opcode: String,
  var: String,
  ins_type: String,
  num: String,
}

impl Instruction {
  fn default() -> Instruction {
    return Instruction {
      opcode: String::new(),
      var: String::new(),
      ins_type: String::new(),
      num: String::new(),
    }
  }
  fn parse_opcode(&mut self, opcode: &str) {
    self.opcode = String::from(OPCODES[opcode]);
  }

  fn parse_opcode_from_line(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() > 0 {
      self.parse_opcode(parts[0]);
    } else {
      println!("ERROR: Provided line didn't contain any opcode");
    }
  }

  fn parse_var(&mut self, var: &str) {
    self.var = String::from(VARS[&var.to_lowercase()]);
  }

  fn parse_var_from_line(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() > 1 {
      self.parse_var(parts[1]);
    } else {
      println!("ERROR: Provided line didn't contain any var");
    }
  }

  fn parse_type(&mut self, raw_type: &str) {
    self.ins_type = String::from(TYPE[raw_type]);
  }

  fn parse_type_from_line(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() > 2 {
      self.parse_type(parts[2]);
    } else {
      println!("ERROR: Provided line didn't contain any type");
    }
  }

  fn parse_num(&mut self, num: &str) {
    let mut number : String = String::new();

    if num.starts_with('-') {
      number = String::from("1");
    }

    number.push_str(&format!("{:08b}", num.parse::<i32>().unwrap()));

    self.num = number;
  }

  fn parse_num_from_line(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() > 1 {
      self.parse_num(parts[3]);
    } else {
      println!("ERROR: Provided line didn't contain any num");
    }
  }

  fn parse(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    self.parse_opcode(parts[0]);
    self.parse_var(parts[1]);
    self.parse_type(parts[2]);
    self.parse_num(parts[3]);
  }

  fn parse_from_vec(&mut self, parts: Vec<&str>) {
    self.parse_opcode(parts[0]);
    self.parse_var(parts[1]);
    self.parse_type(parts[2]);
    self.parse_num(parts[3]);
  }

  fn as_binary(&self) -> String {
    return String::from(format!("{}{}{}{}", self.opcode, self.var, self.ins_type, self.num));
  }

  fn line_to_binary(&mut self, line: String) -> String {
    self.parse(line);

    return self.as_binary();
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  run(file_path);
}

pub fn run(file_path: &str) {
  let mut result: String = String::new();

  if let Ok(lines) = read_lines(file_path) {
      for ok_line in lines {
          if let Ok(mut line) = ok_line {
              line = line.trim().to_string();
              if !line.starts_with(';') && line.len() > 0 {
                  result.push_str(&process_line(line));
                  result.push_str("\n");
              }
          }
      }
  }

  if Path::new("out.bin").exists() {
    fs::remove_file("out.bin").expect("Unable to delete file");
  }

  fs::write("out.bin", result).expect("Unable to write file");
}

fn process_line(line: String) -> String {
  let mut ins = Instruction::default();
  ins.parse_opcode_from_line(line.clone());

  // This leaves things open to having shorter versions of assembly instructions (ex: PRT $0)
  return match ins.opcode {
    _ => ins.line_to_binary(line),
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

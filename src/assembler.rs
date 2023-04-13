mod test;
pub mod vm;

use std::collections::HashMap;
use std::env;
use std::fs::{File, self};
use std::io::{BufRead, BufReader};
use std::path::Path;
use phf::{phf_map};
use regex::Regex;
use substring::Substring;

static OPCODES : phf::Map<&'static str, u8> = phf_map! {
  "PRT" => 0b0000,
  "SET" => 0b0001,
  "ADD" => 0b0010,
  "SUB" => 0b0011,
  "MUL" => 0b0100,
  "DIV" => 0b0101,
  "JMP" => 0b0110,
  "JNP" => 0b0111,
  "EQL" => 0b1000,
  "CBP" => 0b1001,
  "CLP" => 0b1010,
};

static VARS : phf::Map<&'static str,u8> = phf_map! {
  "$0" => 0b0000,
  "$1" => 0b0001,
  "$2" => 0b0010,
  "$3" => 0b0011,
  "$4" => 0b0100,
  "$5" => 0b0101,
  "$6" => 0b0110,
  "$7" => 0b0111,
  "$8" => 0b1000,
  "$9" => 0b1001,
  "$a" => 0b1010,
  "$b" => 0b1011,
  "$c" => 0b1100,
  "$d" => 0b1101,
  "$e" => 0b1110,
  "$f" => 0b1111,
};

static TYPE : phf::Map<&'static str, u8> = phf_map! {
  "0" => 0b00,
  "1" => 0b01,
  "2" => 0b10,
  "3" => 0b11,
};

struct Instruction {
  opcode: u8,
  var: u8,
  ins_type: u8,
  num: u8,
}

impl Instruction {
  fn default() -> Instruction {
    Instruction {
      opcode: 0,
      var: 0,
      ins_type: 0,
      num: 0,
    }
  }
  fn parse_opcode(&mut self, opcode: &str) {
    self.opcode = OPCODES[opcode];
  }

  fn parse_var(&mut self, var: &str) {
    self.var = VARS[&var.to_lowercase()];
  }

  fn parse_type(&mut self, raw_type: &str) {
    self.ins_type = TYPE[raw_type];
  }

  fn parse_num(&mut self, num: &str) {
    if num.starts_with('-') {
      self.num = 0b10000000;
      self.num |= String::from(&num[1..num.len()]).parse::<u8>().unwrap();
    } else {
      self.num = num.parse::<u8>().unwrap();
    }
  }

  fn parse_num_as_var(&mut self, num: &str) {
    self.num = VARS[num];
  }

  fn parse(&mut self, line: String) {
    let parts: Vec<&str> = line.split(' ').collect();

    self.parse_opcode(parts[0]);
    self.parse_var(parts[1]);
    self.parse_type(parts[2]);
    self.parse_num(parts[3]);
  }

  fn as_binary(&self) -> u32 {
    let mut result: u32 = 0;
    // add opcode
    result |= self.opcode as u32;
    result <<= 4;

    // Add var
    result |= self.var as u32;
    result <<= 2;

    // Add type
    result |= self.ins_type as u32;
    result <<= 8;

    // Add num
    result |= self.num as u32;

    result
  }

  fn line_to_binary(&mut self, line: String) -> u32 {
    self.parse(line);

    self.as_binary()
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  run(file_path, "out.bin").unwrap();
}

pub fn run(file_path: &str, output_path: &str) -> std::io::Result<()> {
  let mut label_table = HashMap::new();
  let mut current_line: i32 = 0;
  let re = Regex::new(r"\#\w+").unwrap();

  let lines = lines_from_file(file_path);
  let mut pre_replacement: Vec<String> = Vec::new();
  let mut pre_assembled: Vec<String> = Vec::new();

  // First pass creates the label table
  for line in &lines {
    if line.starts_with('#') {
      label_table.insert(line, current_line);
    } else if !line.starts_with(';') && !line.is_empty() {
      pre_replacement.push(line.to_string());
      current_line += 1;
    }
  }

  for line in &pre_replacement {
    let mut l: String = line.clone();
    for mat in re.find_iter(line) {
      let label: String = line.substring(mat.start(), mat.end()).to_string();
      let line_num = label_table.get(&label).unwrap() + 1;

      l = l.replace(&label, line_num.to_string().as_str());
    }
    pre_assembled.push(l);
  }

  let mut bytes = Vec::new();

  for line in pre_assembled {
    let instruction: u32 = process_line(line);

    for b in instruction.to_be_bytes() {
      bytes.push(b);
    }
  }

  fs::write(output_path, &bytes)?;
  Ok(())
}

fn process_line(line: String) -> u32 {
  let mut ins = Instruction::default();
  let opcode = line.split(' ').collect::<Vec<&str>>()[0];

  // Handle all possibilities of short forms
  match opcode {
    "PRT" => parse_just_var_or_num(line),
    "SET" => parse_no_type(line),
    "JMP" => parse_just_num(line),
    "JNP" => parse_just_num(line),
    "ADD" => parse_no_type(line),
    "SUB" => parse_no_type(line),
    "MUL" => parse_no_type(line),
    "DIV" => parse_no_type(line),
    "EQL" => parse_no_type(line),
    "CBP" => parse_no_type(line),
    "CLP" => parse_no_type(line),
    _ => ins.line_to_binary(line),
  }
}
// Possible ways to use the print operation:
// PRT $x => Just print the variable
// PRT num => Directly print a number
// PRT $x type num => The full version
fn parse_just_var_or_num(line: String) -> u32 {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 2 {
    if parts[1].starts_with('$') {
      ins.parse_var(parts[1]);
      ins.parse_type("2");
      ins.parse_num("0");

      return ins.as_binary();
    } else {
      ins.parse_var("$0");
      ins.parse_type("0");
      ins.parse_num(parts[1]);

      return ins.as_binary();
    }
  }

  // Default
  ins.line_to_binary(line)
}

fn parse_just_num(line: String) -> u32 {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 2 {
    ins.parse_var("$0");
    ins.parse_type("0");
    ins.parse_num(parts[1]);

    return ins.as_binary();
  }

  ins.line_to_binary(line)
}

fn parse_no_type(line: String) -> u32 {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 3 {
    ins.parse_var(parts[1]);
    ins.parse_type("0");

    if parts[2].starts_with('$') {
      ins.parse_type("1");
      ins.parse_num_as_var(parts[2]);
    } else {
      ins.parse_type("0");
      ins.parse_num(parts[2]);
    }

    return ins.as_binary();
  }

  ins.line_to_binary(line)
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
      .map(|l| l.expect("Could not parse line").trim().to_string())
      .collect()
}

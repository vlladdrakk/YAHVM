mod test;
mod vm;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::Path;
use phf::{phf_map};
use regex::Regex;
use substring::Substring;

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

  run(file_path, "out.bin").unwrap();
}

pub fn run(file_path: &str, output_path: &str) -> std::io::Result<()> {
  let mut result: String = String::new();
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
    } else if !line.starts_with(';') && line.len() > 0 {
      pre_replacement.push(line.to_string());
      current_line += 1;
    }
  }

  for line in &pre_replacement {
    let mut l: String = line.clone();
    for mat in re.find_iter(&line) {
      let label: String = line.substring(mat.start(), mat.end()).to_string();
      let line_num = label_table.get(&label).unwrap() + 1;

      l = l.replace(&label, line_num.to_string().as_str());
    }
    pre_assembled.push(l);
  }

  for line in pre_assembled {
    result.push_str(&process_line(line));
    result.push_str("\n");
  }

  let mut file = File::create(output_path)?;
  file.write_all(result.as_bytes())?;
  file.sync_all()?;
  Ok(())
}

fn process_line(line: String) -> String {
  let mut ins = Instruction::default();
  let opcode = line.split(' ').collect::<Vec<&str>>()[0];

  // Handle all possibilities of short forms
  return match opcode {
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
fn parse_just_var_or_num(line: String) -> String {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 2 {
    if parts[1].starts_with("$") {
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
  return ins.line_to_binary(line);
}

fn parse_just_num(line: String) -> String {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 2 {
    ins.parse_var("$0");
    ins.parse_type("0");
    ins.parse_num(parts[1]);

    return ins.as_binary();
  }

  return ins.line_to_binary(line);
}

fn parse_no_type(line: String) -> String {
  let parts: Vec<&str> = line.split(' ').collect();
  let mut ins = Instruction::default();
  ins.parse_opcode(parts[0]);

  if parts.len() == 3 {
    ins.parse_var(parts[1]);
    ins.parse_type("0");
    ins.parse_num(parts[2]);

    return ins.as_binary();
  }

  return ins.line_to_binary(line);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
      .map(|l| l.expect("Could not parse line").trim().to_string())
      .collect()
}

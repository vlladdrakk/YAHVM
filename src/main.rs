mod test;

use std::{env, fs};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use phf::{phf_map};
use std::process::exit;

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
    "$2" => "0001",
    "$3" => "0010",
    "$4" => "0011",
    "$5" => "0100",
    "$6" => "0101",
    "$7" => "0110",
    "$8" => "0111",
    "$9" => "1000",
    "$a" => "1001",
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

    fs::write("out.bin", result).expect("Unable to write file");
}

fn process_line(line: String) -> String {
    let parts: Vec<&str> = line.split(' ').collect();

    if !OPCODES.contains_key(parts[0]) {
        println!("Failed to parse, invalid opcode {}", parts[0]);
        exit(1);
    }

    let opcode = OPCODES[parts[0]];
    let var = VARS[&parts[1].to_lowercase()];
    let types = TYPE[parts[2]];
    let mut num : String = String::new();

    if parts[3].starts_with('-') {
        num = String::from("1");
    }

    num.push_str(&format!("{:08b}", parts[3].parse::<i32>().unwrap()));

    return String::from(format!("{opcode}{var}{types}{num}"));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
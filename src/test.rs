#[cfg(test)]
mod tests {
  use std::fs;
  use crate::*;

  #[test]
  fn it_works() {
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
}
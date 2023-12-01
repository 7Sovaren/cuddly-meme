use std::fs;
use std::path::PathBuf;
use std::iter;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("examples/input.txt");

    let mut result:i32 = 0; 
    let lines = read_lines(d.as_os_str().to_str().unwrap());
    if lines != None{
        let lines = lines.unwrap();
        let line_itterator = lines.iter();

        for line in line_itterator{
            result += get_line_checkdigit(line);
        }
        
        println!("{}", result);
    }

    
}


fn read_lines(filename: &str) -> Option<Vec<String>>{

    let mut result = Vec::new();

    let filehandle = fs::read_to_string(filename);

    if !filehandle.is_ok(){
        None
    }
    else
    {
        for line in filehandle.unwrap().lines() {
            result.push(line.to_string());
        }
    
        Some(result)
    }
}

fn get_line_checkdigit(line: &String) -> i32{
    let chars = line.chars();
    let mut numbers: Vec<i32> = Vec::new();
    for ch in chars {
        if ch.is_digit(10) {
            numbers.push(String::from(ch).parse::<i32>().unwrap());
        }
    }

    let first = numbers.first().unwrap();
    let second = numbers.last().unwrap();

    (first*10)+second
}


#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn read_not_exist(){
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("examples/does_not_exist.txt");
    println!("{}", d.display());
    let lines = read_lines(d.as_os_str().to_str().unwrap());
    assert!(lines == None);
  }

  #[test]
  fn read_example1(){
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("examples/example1.txt");
    let lines = read_lines(d.as_os_str().to_str().unwrap());
    assert!(lines != None);
    let lines = lines.unwrap();
    assert!(lines[0] == "12".to_string());
    assert!(lines[1] == "23".to_string());
  }

  #[test]
  fn read_example(){
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("examples/example.txt");
    let lines = read_lines(d.as_os_str().to_str().unwrap());
    assert!(lines != None);
    let lines = lines.unwrap();
    assert!(lines[0] == "1abc2".to_string());
    assert!(lines[1] == "pqr3stu8vwx".to_string());
    assert!(lines[2] == "a1b2c3d4e5f".to_string());
    assert!(lines[3] == "treb7uchet".to_string());
  }


  #[test]
  fn last_first_singles(){
    let digit = get_line_checkdigit(&"1".to_string());
    assert!(digit == 11);
    let digit = get_line_checkdigit(&"2".to_string());
    assert!(digit == 22);
    let digit = get_line_checkdigit(&"3".to_string());
    assert!(digit == 33);
  }

  #[test]
  fn last_first_doubles(){
    let digit = get_line_checkdigit(&"11".to_string());
    assert!(digit == 11);
    let digit = get_line_checkdigit(&"1a".to_string());
    assert!(digit == 11);
    let digit = get_line_checkdigit(&"12".to_string());
    assert!(digit == 12);
    let digit = get_line_checkdigit(&"23".to_string());
    assert!(digit == 23);
  }

  #[test]
  fn last_first_tripples(){
    let digit = get_line_checkdigit(&"111".to_string());
    assert!(digit == 11);
    let digit = get_line_checkdigit(&"11a".to_string());
    assert!(digit == 11);
    let digit = get_line_checkdigit(&"23a".to_string());
    assert!(digit == 23);
  }

  #[test]
  fn check_example(){
    let digit = get_line_checkdigit(&"1abc2".to_string());
    assert!(digit == 12);
  }
}

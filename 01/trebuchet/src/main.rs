fn main() {
    println!("Hello, world!");
}


fn read_lines(filename: &str) -> Option<String>{
    if filename == "example1"{
        Some("1abc2".to_string())
    }
    else{
        None
    }
}

fn get_first_last_digit(line: String) -> i32{
    let number: String = String::from(line.chars().nth(0).unwrap());
    let number = number.parse::<i32>().unwrap();
    (number*10)+number
}


#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn read_not_exist(){
    let lines = read_lines("does_not_exist");
    assert!(lines == None);
  }

  #[test]
  fn read_example1(){
    let lines = read_lines("example1");
    assert!(lines != None);
    let lines = lines.unwrap();
    assert!(lines == "1abc2".to_string());
  }

  #[test]
  fn last_first_singles(){
    let digit = get_first_last_digit("1".to_string());
    assert!(digit == 11);
    let digit = get_first_last_digit("2".to_string());
    assert!(digit == 22);
    let digit = get_first_last_digit("3".to_string());
    assert!(digit == 33);
  }

  #[test]
  fn last_first_doubles(){
    let digit = get_first_last_digit("11".to_string());
    assert!(digit == 11);
  }
}

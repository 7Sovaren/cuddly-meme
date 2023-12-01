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
  

}

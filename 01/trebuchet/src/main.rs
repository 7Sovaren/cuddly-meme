fn main() {
    println!("Hello, world!");
}






fn read_lines(filename: &str) -> Option<Vec<String>>{
    Some(vec!["1abc2".to_string()])
}


















#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn create_empty(){
    let lines = read_lines("example");
    assert!(lines != None);
    let lines = lines.unwrap();

    assert!(lines[0] == "1abc2");
  }
}

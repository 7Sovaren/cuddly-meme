fn main() {
    println!("Hello, world!");
}






fn read_lines(filename: &str) -> Option<Vec<String>>{
    Some(Vec::<String>::new())
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

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
  fn read_examples(){
    let lines = read_lines("example");
    assert!(lines != None);
    let lines = lines.unwrap();

    assert!(lines[0] == "1abc2");
    assert!(lines[1] == "pqr3stu8vwx");
    assert!(lines[2] == "a1b2c3d4e5f");
    assert!(lines[3] == "treb7uchet");
  }
  

}

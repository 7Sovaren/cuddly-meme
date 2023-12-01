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
    let lines = read_lines("does_not_exist");
    assert!(lines == None);
  }
  

}

fn main() {
    println!("Hello, world!");
}






fn read_lines(filename: &str) -> Option<Vec<String>>{
    None
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
  fn read_example(){
    let lines = read_lines("example");
    assert!(lines != None);
  }
  

}

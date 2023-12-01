fn main() {
    println!("Hello, world!");
}























#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn create_empty(){
    let lines = read_lines("example");
    assert!(lines != NULL);
  }
}

fn coordinates() -> (i32, i32){
  (5, 86)
}

fn main() {
  let (_x, y) = coordinates();

  if y > 5 {
    println!("Y is more than 5");
  } else if y < 5 {
    println!("Y is less than 5");
  } else {
    println!("Y IS 5! :0");
  }
}
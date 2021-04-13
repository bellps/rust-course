fn sum(a: i32, b: i32) -> i32{
  a + b
}

fn show_result(result: i32){
  println!("The result is {:?}", result);
}

fn main(){
  let result = sum(10, 5);
  show_result(result);
}
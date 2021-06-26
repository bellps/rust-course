fn print_size(my_bool: bool) {
  match my_bool {
    true => println!("Its biiig"),
    false => println!("Is smoll boi :("),
  }
}

fn main() {
  let my_var: i32 = 101;

  let is_big: bool = my_var > 100;
  print_size(is_big);
}
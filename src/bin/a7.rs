enum Color {
  Pink,
  Blue,
  Yellow,
  Brown,
  Magenta
}

fn print_color(color: Color) {
  match color {
    Color::Pink => println!("pink!"),
    Color::Blue => println!("bluuuue"),
    Color::Yellow => println!("Yellow :D"),
    Color::Brown => println!("brownie hmmm"),
    Color::Magenta => println!("magenta.")
  };
}

fn main() {
  let new_color = Color::Magenta;

  print_color(new_color);
}
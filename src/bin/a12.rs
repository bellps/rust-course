enum Color {
  Blue,
  Yellow,
  Green,
  Red,
}

struct ShippingBox {
  width: f64,
  height: f64,
  weight: f64,
  color: Color,
}

impl ShippingBox {
  fn new(width: f64, height: f64, weight: f64, color: Color) -> Self {
    Self {
      width,
      height,
      weight,
      color,
    }
  }

  fn print_box(&self) {
    print!("It is a box of {:?} pounds, dimentions {:?}x{:?} and have the color ", self.weight, self.width, self.height);
    match self.color {
      Color::Blue => println!("blue"),
      Color::Yellow => println!("yellow"),
      Color::Green => println!("green"),
      Color::Red => println!("redd"),
    };
  }
}

fn main() {
  let my_box = ShippingBox::new(12.8, 3.7, 5.0, Color::Red);
  my_box.print_box();
}
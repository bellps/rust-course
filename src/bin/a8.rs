enum DrinkFlavor {
  Sweet,
  NotThatSweet,
  Orange,
}

struct Drink {
  flavor: DrinkFlavor,
  volume: f64,
}

fn print_drink(drink: Drink) {
  print!("Drink flavor: ");

  match drink.flavor {
    DrinkFlavor::Sweet => println!("sweet"),
    DrinkFlavor::NotThatSweet => println!("nooot that sweet"),
    DrinkFlavor::Orange => println!("orange"),
  };

  println!("Volume: {:?} oz", drink.volume);
}

fn main() {
  let new_drink = Drink {
    flavor: DrinkFlavor::Sweet,
    volume: 27.5
  };

  print_drink(new_drink);
}
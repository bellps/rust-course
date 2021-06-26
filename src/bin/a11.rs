struct GroceryItem {
  id: i32,
  quantity: i32,
}

fn print_qtt(item: &GroceryItem) {
  println!("The quantity is {:?}", item.quantity);
}

fn print_id(item: &GroceryItem) {
  println!("The id of the item is {:?}", item.id);
}

fn main() {
  let apple = GroceryItem {
    id: 4,
    quantity: 45,
  };

  print_qtt(&apple);
  print_id(&apple);
}
fn main(){
  let mut my_var = 1;

  loop{
    if my_var == 5{
      break;
    }

    println!("{:?}", my_var);
    my_var += 1;
  }
}
fn main() {
  let x_i32: i32 = 1;
  let y_i32: i32 = -1;
  println!("{}", x_i32 + y_i32);

  let result = add(1, 1);
  println!("{}", result);

  let x = {
    let y = 1;
    y + 1
  };
  println!("{}", x);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}

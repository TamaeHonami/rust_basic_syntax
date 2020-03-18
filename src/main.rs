extern crate rust_lib_example;

struct User {
  first_name: String,
  family_name: String,
  age: u8,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // More parameter
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Associated Functions
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddrKind {
  fn notmeaning(&self) {
    println!("{:?}", self);
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

use Coin::*;
use rust_lib_example::nest::nest_module;

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

  let condition: bool = true;
  if condition {
    println!("true");
  } else {
    println!("false");
  }

  let arr = [10, 20, 30, 40, 50];
  for element in arr.iter() {
    println!("{}", element);
  }

  let test1: i32 = 5;
  let test2: i32 = test1;
  println!("{}", test1);
  println!("{}", test2);

  {
    let s1 = String::from("hello");
    println!("s1 {}", s1);
    let s2 = s1;
    println!("s2 {}", s2);
    // One more s1 print NG!!
    //println!("s1 {}", s1);
  }

  let s3: String = String::from("from");
  let len: usize = get_length(&s3);
  println!("{} {}", s3, len);

  let mut s4: String = String::from("hello");
  change(&mut s4);
  println!("{}", s4);

  let my_string: String = String::from("Hello world!");
  let word: &str = first_word(&my_string[..]);
  println!("{}", word);

  let my_str_literal: &str = "Hello world!";
  let word: &str = first_word(&my_str_literal[..]);
  println!("{}", word);
  // or
  let word: &str = first_word(my_str_literal);
  println!("{}", word);

  let arr2: [i32; 5] = [1, 2, 3, 4, 5];
  let arr_slice: &[i32] = &arr2[1..4];
  for elem in arr_slice.iter() {
    println!("{}", elem);
  }

  let tama: User = User {
    first_name: String::from("Tamae"),
    family_name: String::from("Honami"),
    age: 12,
  };
  println!("{} {} ({})", tama.first_name, tama.family_name, tama.age);

  let father: User = User {
    first_name: String::from("Shintarou"),
    ..tama
  };
  println!("{} {} ({})", father.first_name, father.family_name, father.age);

  let rect1: Rectangle = Rectangle { width: 30, height: 50 };
  println!("{:#?}", rect1);
  println!("{}", rect1.area());

  let rect2: Rectangle = Rectangle::square(20);
  println!("{}", rect1.can_hold(&rect2));

  let ipv4_localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
  ipv4_localhost.notmeaning();
  let ipv6_loopback: IpAddrKind = IpAddrKind::V6(String::from("::1"));
  ipv6_loopback.notmeaning();

  let some_number = Some(32);
  let none_number: Option<i32> = None;
  println!("{:?}", some_number);
  println!("{:?}", none_number);

  println!("{}", value_in_cents(Coin::Penny));
  println!("{}", value_in_cents(Coin::Nickel));
  println!("{}", value_in_cents(Coin::Dime));
  println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
  println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

  let five: Option<i32> = Some(5);
  let six: Option<i32> = plus_one(five);
  println!("{:?}", six);

  let some_u8_value: Option<u8> = Some(3u8);
  if let Some(3) = some_u8_value {
    println!("three");
  }

  rust_lib_example::nest::nest_module::test_func();
  nest_module::test_func();
  rust_lib_example::client::connect();
  println!("{}", value_in_cents(Penny));

  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v.push(4);
  println!("{:?}", v);

  let v2 = vec![5, 6, 7];
  println!("{:?}", v2);

  let second: &i32 = &v2[1];
  println!("{}", second);
  let third: Option<&i32> = v.get(2);
  println!("{:?}", third);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn get_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str(", world!");
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]
    }
  }
  &s[..]
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("Penny");
      1
    },
    Coin::Quarter(state) => {
      println!("{:?}", state);
      25
    },
    _ => {
      println!("Other coin.");
      0
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

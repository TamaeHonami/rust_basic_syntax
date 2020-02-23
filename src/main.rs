struct User {
  first_name: String,
  family_name: String,
  age: u8,
}

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

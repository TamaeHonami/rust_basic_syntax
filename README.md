# Rust basic syntax

## Summary

Rustの基本構文についてまとめたメモ.  

## Text

### Variables

変数定義の仕方.  

```
fn main() {
  let x = 0;
}
```

変数は, デフォルトではイミュータブルとして扱われる.  
ミュータブルとして扱う場合は, `mut`を付ける.  

```
fn main() {
  let mut x = 0;
  x = 1;
}
```

イミュータブルが基本になってるのいいね!!

注釈  
`Shadowing`と呼ばれる機能があり, 下記のような記述は許可されている.  

```
fn main() {
  let x = 0;
  let x = x + 1;
  let x = x * 2;
}
```

再定義みたいなものか???  
ちなみに束縛する型を変えることもできる.  

### Constants

定数定義の仕方.  

```
const MAX_POINTS: u32 = 100_000;
```

### Data type

Rustは静的型付き言語である.  
コンパイル時には全ての型が判明している必要がある.  

大きく分けて2つの型がある.  

- スカラ型
  - 整数型
    - i8
    - u8
    - i16
    - u16
    - i32
    - u32
    - i64
    - u64
    - isize
    - usize
  - 浮動小数点型
    - f32
    - f64 (default)
  - 論理値型
    - bool
  - 文字型
    - char
- 複合型
  - タプル型
  - 配列型

#### 整数リテラル例

- 10進数: 10_000
- 16進数: 0xff
- 8進数: 0o77
- 2進数: 0b1111_0000
- バイト(u8): b'A'

#### 加減剰余

```
fn main() {
  let add = 1 + 1;
  let sub = 1 - 1;
  let mul = 1 * 1;
  let div = 1 / 1;
  let rem = 1 % 1;
}
```

#### 色々サンプル

```
fn main() {
  // 整数型
  let x: u32 = 1;
  let y: i32 = -1;
  // 不動小数点型
  let float32: f32 = 1.0;
  let float64: f64 = 1.0;
  // 論理型
  let bool_t = true;
  let bool_f: bool = false;
  // 文字型&文字列
  let ch: char = 'a';
  let s: &str = "abc";
  let cs: String = "hello";
  // タプル型
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  let one = tup.2;
  // 配列型
  let arr = [1, 2, 3, 4, 5];
  let first = arr[0];
}
```

### Function

関数定義の仕方.  

```
fn main() {
  call_func();
  add(1, 1);
}

fn call_func() {
  println!("Hello, world!!");
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

**注意: 式の終端にセミコロン(;)が付くと文として扱われる!!**  

Rustは式指向言語のため, こういうこともできる.  

```
fn main() {
  let x = {
    let y = 1;
    y + 1
  };
}
```

### Comment

Rustでのコメントの仕方は次の通り.  

- ラインコメント

```
// コメント
fn main() {
  // コメント
  println!("Hello, world!"); // コメント
}
```

- ブロックコメント

```
fn main() {
  let x: i32 = 32;
  /* "ブロックコメント開始"
  let y: i32 = 32;
  let z: i32 = 32;
  "ブロックコメント終了" */
  println!("{}", x);
}
```

- 内部ドキュメントコメント

```
mod foo {
  //! 内部コメント1
  //! 内部コメント2
  //! 内部コメント3
  struct Bar {
    pub baz: i32
  }
}
```

- 外部ドキュメントコメント

```
/// 外部コメント1
/// 外部コメント2
/// 外部コメント3
pub enum Result {
  Success,
  Warning,
  Failure(Error)
}
```

**耳寄り情報: ドキュメントコメントはマークダウンがサポートされている.**  
下記のコマンドでドキュメントを出力できる.  

```
$ cargo doc
```

### Control flow

フローの制御方法について記載する.  

#### 分岐処理

- if

```
fn main() {
  let condition: bool = true;
  if condition {
    println!("true");
  } else {
    println!("false");
  }
}
```

```
fn main() {
  let number: i32 = 0;
  if number == 0 {
    println!("number is zero.");
  } else if number == 1 {
    println!("number is one.");
  } else {
    println!("other number.");
  }
}
```

```
fn main() {
  let condition: bool = true;
  let number: i32 = if condition {
    0
  } else {
    1
  };
}
```

#### 繰返し処理

- loop

```
fn main() {
  loop {
    println!("loop!!");
  }
}
```

- while

```
fn main() {
  let mut count: i32 = 0;
  while count <= 3 {
    println!("{}", count);
    number = number + 1;
  }
}
```

- for

```
fn main() {
  let arr = [0, 1, 2, 3, 4, 5];

  let mut index: i32 = 0;
  while index <= 5 {
    println!("{}", arr[index]);
    index = index + 1;
  }

  for element in arr.iter() {
    println!("{}", element);
  }
}
```

### Ownership

Rustの中心的機能で所有権と言われる機能についてまとめる.  

- 規則
  - 各値は所有者と呼ばれる変数と対応している.
  - いかなる時も所有者は1つとなる.
  - 所有者がスコープから外れたら値は破棄される.

- ムーブ

下記のようにヒープメモリへ積まれる場合には, s1変数は無効となる.  
メモリの二重解放にならないようにするための仕組みとのこと.  
(これが"所有者はいかなる時も1つ"ということ???)

```
let s1: String = String::from("hello");
let s2: String = s1;

println!("{}", s1);
```

下記はスタックメモリへ積まれるためOK.  
(コンパイル時に既知のサイズを持つ型はこっちになる.)  

```
let x: i32 = 5;
let y: i32 = x;
```

- クローン

ディープコピーしたい場合はcloneメソッドを使う.  

```
let s1: String = String::from("hello");
let s2: String = s1.clone();

println!("{}", s1);
```

**Copyトレイトについて**
一般的なスカラ型やその集合はCopyトレイトが使われるため, いちいちcloneメソッドを使わなくてよい.  
メモリ確保が必要だったり, 何らかの形態のリソースだったりした場合はだめ.  
また, 型やその一部でもDropトレイトが実装されている場合, Copyトレイトは使えない.  

**関数や戻り値では???**
基本的には関数の引数として変数を渡した場合,  
Copyトレイトが使えないものはムーブで渡されるため, 渡した後に使用することはできない.  
また, 戻り値として返ってくるものも同じ.  

ただし, 参照と借用を利用した場合は別の動作をさせることができる.  
(引き回して使いたい場合もあるしね.)  

アンパサンド(&)を付けることによって参照として渡すことができる.  
(参照外しと呼ばれる逆パターンの機能も存在する.)

```
fn main() {
  let s1: String = String::from("hello");
  let len: usize = get_length(&s1);
  println!("{} {}", s1, len);
}

fn get_length(s: &String) -> usize {
  s.len()
}
```

このように関数の引数に参照を取ることを借用と言う.  
借用したものは必ず返さなければいけない.  
また, 変数と同じく参照もデフォルトで不変であるため, 上記のようなコードの関数内(get_length)で引数の値を変更することはできない.  
借用規則として, 何かへの不変な参照がある場合, 可変な参照は取得することができない点も合わせて.  

もし, 参照しているものを可変として扱いたければ, `&mut` を使う.  

```
fn main() {
  let mut s1: String = String::from("hello");
  change(&mut s1);
  println!("{}", s1);
}

fn change(s: &mut String) {
  s.push_str(", world!");
}
```

ただし制約も存在する.  
特定のスコープにおいて, ある特定のデータに対し, 1つしか可変な参照は持てない.  
データ競合を起こさないために, この制約が存在する.  

### Struct

構造体についてまとめる.  
構造体は複数の関連あるデータ型を一つにまとめて定義することができる機能のこと.  

構造体の定義

```
struct User {
  first_name: String,
  family_name: String,
  age: u8,
}

fn main() {
  let user: User = User {
    first_name: String::from("Tamae"),
    family_name: String::from("Honami"),
    age: 12,
  };

  println!("{} {} ({})", user.first_name, user.family_name, user.age);
}
```

構造体更新記法

```
struct User {
  first_name: String,
  family_name: String,
  age: u8,
}

fn main() {
  let user1: User = User {
    first_name: String::from("Tamae"),
    family_name: String::from("Honami"),
    age: 12,
  };

  let user2: User = User {
    first_name: String::from("Shintarou"),
    ..user1
  };

  println!("{} {} ({})", user2.first_name, user2.family_name, user2.age);
}
```

タプル構造体

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black: Color = Color(0, 0, 0);
let origin: Point = Point(0, 0, 0);
```

ユニット様構造体というのもあるとのこと.  
その他, 所有権に関しては後述する.  

Example

```
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn area(rect: &Rectagle) -> u32 {
  rect.width * rect.height
}

fn main() {
  let rect: Rectangle = Rectangle {width: 30, height: 50};
  println!("{:#?}", rect);
  
  let pixel: u32 = area(&rect);
  println!("{}", pixel);
}
```

## Extra

### Stringと文字列リテラルについてのメモ

String型は可変, 文字列リテラルは不変.  
また, 文字列リテラルはコンパイル時にバイナリ内でハードコードされるため, 高速で効率的になる.  

### スライス

スライスはコレクションの一部を参照することができるようになる.  

- 文字列スライス

```
fn main() {
  let s1: String = String::from("Hello world!");
  let word: &str = first_word(&s1[..]);

  let str_literal &str = "Hello world!";
  let word: &str = first_word(&str_literal[..]);
  // or
  let word: &str = first_word(str_literal);
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
```

- 配列のスライス

```
fn main() {
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  let slice: &[i32] = &arr[1..3];
}
```

## Reference



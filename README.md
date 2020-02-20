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

## Extra



## Reference



/*
avoid keywords "_"
*/
use std::any::type_name;

fn printtype<T>(_:T){
  println!("{}", type_name::<T>());
}

/*
//issue:why cant execute?
fn printtype<T>(typetest: T){
  println!("{}", type_name::typetest());
}
*/

fn main(){
  //token type check
  let character_ = 'H';
  println!("{}", character_);
  printtype(character_);
  let string_ = "hello";
  println!("{}", string_);
  printtype(string_);
  let rawstring_ = r#"hello"#;
  println!("{}", rawstring_);
  printtype(rawstring_);
  let byte_ = b'H';
  println!("{}", byte_);
  printtype(byte_);
  let bytestring_ = b"Hello";
  
  //このパターンは要素番号文字目の文字のバイト型が出力される
  println!("{}", bytestring_[0]);
  printtype(bytestring_);
  let rawbytestring_ = br#"hello"#;
  println!("{}", rawbytestring_[0]);
  printtype(rawbytestring_);
  
  //98_222の_は視覚的に見やすいようにつけることができる
  //どのような数値型でもこれは使用可能
  //98222でも98_222でも同じく98222が数値として代入される
  let decimal_ = 98_222;
  println!("{}", decimal_);
  printtype(decimal_);
  //16進数でff(255[decimal])
  let hexint_ = 0xff;
  println!("{}", hexint_);
  printtype(hexint_);
  //8進数の77(63[decimal])
  let octalint_ = 0o77;
  println!("{}", octalint_);
  printtype(octalint_);
  //2進数で11110000(240[decimal])
  let binint_ = 0b1111_0000;
  println!("{}", binint_);
  printtype(binint_);
  //float表記
  //123.0 * 10^77
  let floatpoint_ = 123.0E+77;
  println!("{}", floatpoint_);
  printtype(floatpoint_);
}

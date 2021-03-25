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
  printtype(character_);
  let string_ = "hello";
  printtype(string_);
  let rawstring_ = r#"hello"#;
  printtype(rawstring_);
}

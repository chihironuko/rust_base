fn main() {
    //これは文字列リテラルらしい...
    //多分string型ではなく、&strになってる
    let s = "hello"; //ここからsが有効
    println!("{}", s);

    let s = String::from("hello");
    println!("{}", s);

    let mut s = String::from("hello"); //この時点でメモリをOSへ要求
    s.push_str(", world!");
    println!("{}", s);
    
    //move and copy
    //数値型等の既知の固定サイズの変数では、値のコピーが行われて代入が行われる
    let x = 5;
    let y = x;
    println!("{}", y);

    //String型の場合そう簡単ではなく、サイズが不明なため二重でメモリを確保するのではなく、
    //ヒープに確保したメモリ位置のdeep copyになる
    //そうなるとメモリを二重解放する危険があるため、元々のs1変数を破棄して、s2のみ残すようになっている
    //これがmove
    let s1 = String::from("hello");
    let s2 = s1;
    //以下コードは動かない
    //println!("{}, world!", s1);
    //こっちが正解
    println!("{}, world!", s2);

    //clone
    //明示的にdeep copy(参照渡しではなく、同じデータの複製)を行うことができる
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1.push_str(", world!");
    println!("s1 = {}, s2 = {}", s1, s2);

    //ownership
    let s = String::from("hello");
    //ここで行われるのはmove
    takes_ownership(s);
    //この箇所でsはdropされているので、使用できない
    //println!("{}",s);

    let x = 5;
    //ここで行われるのはcopy
    makes_copy(x);
    //intはcopyなので、使用可能
    println!("{}", x);


    //return and scope
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); //s2は関数へムーブ、s3へ関数からムーブ
    println!("{}", s3);
    //これは使用不可能
    //println!("{}", s2);

    //参照(shallow copy)が欲しくなる関数例
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2 = {}, len = {}", s2, len);

    //不変な参照渡し(借用)
    let s1 = String::from("hello");
    //&s1にすることで参照渡しになる
    //s1の所有権を渡すわけではないため、drop処理で死なない
    let len = calculate_length_fix(&s1);
    println!("The length of '{}' is {}.", s1, len);
    //借用エラー
    let s = String::from("hello");
    change_err(&s);

    //可変な参照
    //sをmutableに
    let mut s = String::from("hello");
    //可変な参照は "&mutable型 変数名"
    change(&mut s);
    //もちろん、呼び出した関数先で加えられた変更は、参照元の変数にも加えられている
    println!("shallowed: {}", s);

    //可変な参照の制約
    //ある変数の参照は一度に一つしか作ることができない
    let mut s = String::from("hello");
    //これは可能
    change(&mut s);
    change(&mut s);

    //これはダメ
    //一つ目の参照作成
    //let r1 = &mut s;
    //一つ目が死ぬ前に二つ目の作成(エラー)
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);

    //細かい参照
    //複数の可変の参照
    let mut s = String::from("hello");
    {
        //一つ目の参照が存在
        let _r1 = &mut s;
    }//ここでr1がドロップ
    //二つ目ではなく、一つ目の参照になる
    let _r2 = &mut s;

    //可変な参照と不変な参照の同居(エラー)
    let mut _s = String::from("hello");
    let _r1 = &s;
    let _r2 = &s;
    //これはエラー
    //可変な参照と不変な参照が同時に存在することはできない
    //let r3 = &mut s;

    //ダングリングポインタ
    //他に参照を渡した後に参照元のメモリを解放してしまうことで発生するエラー
    //let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing: {}", reference_to_nothing);

    let splitwords = String::from("goodbye hello world.");
    let word = first_word(&splitwords);
    //splitwordsはfirst_wordに参照を不変として渡しており、clear処理は可変変数出なくては実行できないため、エラーになる
    //splitwords.clear();
    println!("{}", word);

    //スライス
    //[..]で文字列全体のスライスにもなる
    let s = String::from("hello world");
    //[..5]で表記してもよい
    let hello = &s[0..5];
    //[6..]で表記してもよい
    let world = &s[6..11];
    println!("{}",hello);
    println!("{}",world);

    //ここで、sは&str型であり、不変な参照である
    //let s = "Hello, world!";

    //Stringのスライス
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}",word);
    //文字リテラルのスライス
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);
    //元からリテラル(&str)なので、&をつけずともコンパイル可能
    let word = first_word(my_string_literal);
    println!("{}",word);
    
    //slice(another)
    let a = [1,2,3,4,5];
    //aの参照、1-2まで
    let slice = &a[1..3];
    println!("all slice: {}, {}", slice[0], slice[1]);

}//ここでsがdrop
//有効でなくなったタイミングでOSへメモリを返還する
//ある変数がスコープを抜ける際に、dropと呼ばれる特別な関数が呼ばれ、メモリが返還される
//この閉じ括弧もdropを呼ぶコード

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}//drop: some_string

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}//drop: some_integer

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    //ここはmove
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_fix(s: &String) -> usize {
    s.len()
}//ここのdropでsは死ぬが、参照を持っているだけで所有権がないため関数に渡された参照元の変数が死ぬことはない。

fn change_err(some_string: &String) {
    //参照を借りているだけなので、参照元の変数に変更を加えようとしてエラーが出る。
    //some_string.push_str(", world!");
    println!("{}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
    println!("changed: {}", some_string);
}

/*宣言する時点でエラーを吐かれる
返値が参照な時点でだめっぽい
fn dangle() -> &String {
    let s = String::from("hello");
    &s //ここで参照を返す
}//しかし、ここで参照を返した参照元の変数sをdropするため、ダングリングが起きる。
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

//受け取る値を文字リテラルにすることで、スライスもStringもどちらが送られてきても対応可能となる
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

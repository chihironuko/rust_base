# RustBase  
私のRust学習帳(基礎)  
Rustはキャメルケースの書き方なので、RustBase。  

## Documents  
英語リファレンス   :https://doc.rust-lang.org/stable/reference/introduction.html  
日本語リファレンス :https://doc.rust-jp.rs/book-ja/title-page.html  
英語は完全にリファレンス、日本語はチュートリアルも含めた感じの文書だった。  

英語ドキュメント紹介  :https://www.rust-lang.org/learn  
日本語ドキュメント紹介:https://doc.rust-jp.rs/  
どちらもまだ見られていない。  

## 把握すべき特徴  
- 式と文  
[英語リファレンス](https://doc.rust-lang.org/stable/reference/statements-and-expressions.html)  
[日本語リファレンス](https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html#%E9%96%A2%E6%95%B0%E6%9C%AC%E4%BD%93%E3%81%AF%E6%96%87%E3%81%A8%E5%BC%8F%E3%82%92%E5%90%AB%E3%82%80)  
基本的に文。何かを評価したら式。  
大きな特徴としては、式の末尾にはセミコロンがつかないらしい?  
```rust
//日本語リファレンスより。
fn main() {
  //スコープ範囲がlet y = {};内に届かないことを示す。
  let x = 5;

  //変数yに{}式内で評価された値を挿入する文
  let y = {
    let x = 3;
    //x+1を評価する式(返す?)
    x + 1
  };
  //一連の流れは文となり{}内は式だが、{}によって評価された値がyに入れられる部分は文
  println!("The value of y is: {}", y);
}
```

- 変数  
[英語リファレンス(ちょっと変数とは違うけど近しい)](https://doc.rust-lang.org/stable/reference/tokens.html)  
[日本語リファレンス](https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html)  
変数は標準が不変。  
可変の変数はmutをつける。  
変数の型は後から変更することはできない。再宣言が必要になる。  
```rust
//int型、2が入り不変
let a = 2;
//int型、1が入り、可変(再代入可)
let mut b = 1;
//これはできない。
b = "wasd";
//これはできる。
let a = "wasd";
```

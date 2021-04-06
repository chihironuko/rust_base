# RustBase  
私のRust学習帳(基礎)  
Rustはキャメルケースの書き方なので、RustBase。  
Rustは式指向言語。  

## Documents  
英語リファレンス   :https://doc.rust-lang.org/stable/reference/introduction.html  
日本語リファレンス :https://doc.rust-jp.rs/book-ja/title-page.html  
英語は完全にリファレンス、日本語はチュートリアルも含めた感じの文書だった。  

英語ドキュメント紹介  :https://www.rust-lang.org/learn  
日本語ドキュメント紹介:https://doc.rust-jp.rs/  
どちらもまだ見られていない。  

Rustコーディングガイド :https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md  
なんとtabは4スペースらしい。2じゃだめだと。  

## 把握すべき特徴  
- ケース書き分け  
キャメルケースとスネークケースが混在する。  
[参照](https://sinkuu.github.io/api-guidelines/naming.html)  

- 式と文  
[英語リファレンス](https://doc.rust-lang.org/stable/reference/statements-and-expressions.html)  
[日本語リファレンス](https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html#%E9%96%A2%E6%95%B0%E6%9C%AC%E4%BD%93%E3%81%AF%E6%96%87%E3%81%A8%E5%BC%8F%E3%82%92%E5%90%AB%E3%82%80)  
基本的に文。何かを評価したら式。  
大きな特徴としては、式の末尾にはセミコロンがつかないらしい?  
日本語のThe Rust Programming Languageによると、なんらかの動作を行って値を返さない命令が文、結果値に評価されるのが式らしい。  
関数定義も文。定義自体は評価しないため。  

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

- クレート  
コンパイルが行われる単位。  
モジュール等が独立にコンパイル<F3>荒れることはないが、それぞれのクレートは互いに独立にコンパイルされる。  

## 注意点  
- 日本語文字列について  
コンパイルする際にソースコードのコメント中以外に日本語を入力(出力文字列等)すると、コンパイルに失敗することがあるらしいため、できるだけ英語で記述を行う。  
[注釈: The programming language Rust第1版の翻訳者によると、 ソースコードのコメント中以外に日本語文字があるとコンパイルに失敗することがあるそうなので、文字列の英語は、コメントに和訳を載せます。 また、重複する内容の場合には、最初の1回だけ掲載するようにします。](https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html)  




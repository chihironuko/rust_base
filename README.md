# RustBase  
私のRust学習帳(基礎)  
Rustはキャメルケースの書き方なので、RustBase。  
Rustは式指向言語。  
[stability without stagnation](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#stability-without-stagnation): 停滞しない安定性の提供がモットー。  

## Documents  
英語リファレンス   :https://doc.rust-lang.org/stable/reference/introduction.html  
日本語リファレンス :https://doc.rust-jp.rs/book-ja/title-page.html  
英語は完全にリファレンス、日本語はチュートリアルも含めた感じの文書だった。  

英語ドキュメント紹介  :https://www.rust-lang.org/learn  
日本語ドキュメント紹介:https://doc.rust-jp.rs/  
どちらもまだ見られていない。  

Rustコーディングガイド :https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md  
なんとtabは4スペースらしい。2じゃだめだと。  

## rustのバージョン管理について  
rustupを使っている。最も推奨されている。  

## rustコンパイラサポート  
[reference](https://doc.rust-lang.org/nightly/rustc/platform-support.html)  
コンパイラの対応するプラットフォームは"確実に動作する"、Tier1から"ベストエフォート"のTier3までの段階がある。  

## rustのリリースバージョン  
- stable  
安定版。プロジェクト作成に向く。  

- beta  
リリース予定、リリース前のバージョン。バク探しに役立ちたい時に使ってみる。  

- nightly  
不安定版。バグが多かったり、こんな機能どう？がふんだんにある感じっぽい？  

## 曖昧  
- 文字列リテラルとString型  
違いがあるらしい。後ほど調べる。  

## 把握すべき特徴  
- ケース書き分けと記法  
キャメルケースとスネークケースが混在する。  
[参照](https://sinkuu.github.io/api-guidelines/naming.html)  
あと、結局何故か分からなかったが、
```rust  
let s = "hello";
```
のみのコードを作成してコンパイルした際に、
help: if this is intentional, prefix it with an underscore: `_s`  
が出力され、sを  
```rust  
let _s = "hello";
```
とするとwarningが消えた。println等で出力されていない変数にこの警告が出るらしいが、詳細がわかっていない。  

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

- copyとclone  
あらゆる整数型、論理値型、浮動小数点型、文字型(char)、これら4つのみからなるタプルはコピーになる。  
それ以外(String等)はコピーを作成しようとするとmoveになり、コピーを作成する際にはcloneを用いる。  
また、コピーはデフォルトでdeep copy。shallow copyを行う方法は知らない。多分ない。  

- 変数とdrop  
}等、関数の終わりのタイミングなどでそのスコープ内で所有権のある変数のdrop(メモリ返還)が行われる。  
参照を渡された場合は、所有権は移らないためdropしないが、ムーブの場合は所有権が移るためdropする。  

- クレート  
コンパイルが行われる単位。  
モジュール等が独立にコンパイル<F3>荒れることはないが、それぞれのクレートは互いに独立にコンパイルされる。  

## 注意点  
- 日本語文字列について  
コンパイルする際にソースコードのコメント中以外に日本語を入力(出力文字列等)すると、コンパイルに失敗することがあるらしいため、できるだけ英語で記述を行う。  
[注釈: The programming language Rust第1版の翻訳者によると、 ソースコードのコメント中以外に日本語文字があるとコンパイルに失敗することがあるそうなので、文字列の英語は、コメントに和訳を載せます。 また、重複する内容の場合には、最初の1回だけ掲載するようにします。](https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html)  



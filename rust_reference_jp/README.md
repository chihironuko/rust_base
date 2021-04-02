# Rust Reference JP  
日本語版リファレンスを追う。  

## next  
[数あてゲーム](https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html)  

##cargoと実行  
cargoでbuildした際は、実行ファイルはtarget/debugの場所にproject名を名称として作られる  
当該実行ファイルを"./target/debug/project_name"で実行するか、"cargo run"で実行する。  
ちなみにcargo runはコンパイルも同時に行ってくれる優れたコマンド。  
また、実行可能ファイルを生成せずコンパイルが通るかの確認だけしたいときは"cargo check"。  
cargo buildより結構早くなりやすいらしい。(ファイル生成処理がないため。)  
cargo build に --releaseつけるとリリース版のビルドが可能。  
最適化を行ってくれるらしい。このコマンドの場合実行ファイルはtarget/releaseに作られる。  



//ライブラリの読み込み
//標準ではpreludeに存在する型のみ使用可能
//それ以外のライブラリの読み込みの際にuseを用いる
//prelude:https://doc.rust-lang.org/std/prelude/index.html

use std::io;
use rand::Rng;
//バージョンが違うためか、Orderingのuseは必要ないといわれる
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        //&付きは参照。ここを&guessだけにするとエラーが出力される。
        //明確に不変可変を示さないとエラーが出やすいっぽい？
        //この場合はmutableな変数
        //また、rust記法は一行に一回のメソッド呼び出しなので、二行にする。
        //io::stdin().readline()の返値はio::Resultであり、これは列挙型(enum)
        //列挙子はOkかErrとなる
        //io::Resultはexpectがあり、readlineがErrを返した場合(io::ResultがErr)、expectによってメソッドがクラッシュ、
        //引数として渡されたメッセージを表示する
        //Okだった場合、expectはOk列挙子が保持する返値を取り出してその値を返す
        //要するに、メソッドを読んだ際にエラーが起きた場合にクラッシュさせたければexpectを呼んだ方がいい。
        //今回の場合も呼ばなくてもコンパイルも実行もできるが、警告が出る。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //test
}

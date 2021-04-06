fn main() {
    println!("Hello, world!");
    //関数名()で関数の呼び出し。
    another_function();
    //引数は()内に記述
    fn_args(5);
    //複数の引数を渡す際は","で区切る
    multi_args(10,3.2);

    //これは文
    //let y = 6;
    //例として、
    //let x = (let y = 6);
    //の場合、let y = 6は何も返さないため文であり、エラーが出力される。
    //()の中には基本値を返すものが必要。
    //簡単な演算(11 + 4)は、15という値に評価される式なので、先ほどの例の()に入る。

    let yy = {
        //上のxxのスコープ内になるため、シャドーイング。
        //正直見本のために上で宣言しているが、上のxx必要ない。
        let xx = 3;
        xx + 1
    };

    println!("The Value of yy is: {}", yy);
    //この箇所での下の文のxxは、スコープ外となるためエラーが出力される。
    //println!("The Value of xx is: {}", xx);

    let ret = return_function();
    println!("The Value of ret is: {}", ret);

    let adding = plus_one(5);
    println!("The Value of adding is: {}", adding);
}

//関数名はスネークケース(全文字小文字、単語間スネークケース)
//当該関数が呼ばれている箇所の上でも下でもどこで宣言してもよい。コンパイラにとって重要なのは、宣言されているか否か。
fn another_function(){
    println!("Another function.");
}

//関数と引数
//引数の型は必ず宣言する(この場合はint32(符号あり32bit整数))
fn fn_args(uno: i32){
    println!("fn_args The Value of uno is: {}", uno);
}

//複数の引数は、","で区切る
fn multi_args(uno:i32, dos:f64){
    println!("multi_args The Value of uno is: {}", uno);
    println!("multi_args The Value of dos is: {}", dos);
}

fn return_function() -> i32 {
    //;を記述していないため下の5は文ではなく式となり、返値として正しい表記になる。
    5
}

fn plus_one(x: i32) -> i32{
    //下の一文のみの場合、;付きでコンパイラに文だと解釈されるため、返値がi32であると定義されている本関数ではエラーとなる
    //もちろん、他で返値を書いておけば何も文句は言われないが。
    //x + 1;
    //こっちが正解
    x + 1
}

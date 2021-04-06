//constはグローバルスコープも行ける
//また、型指定が厳しく、mutをつけることはできない。
//ついでに、定数は大文字と_で記述するのがrustのルールなので、小文字で作ろうとするとエラーは出ないが警告が出る。
const CONST_X: u32 = 100;
//letはグローバルスコープでは行けない。定数との違いはここ。
//let test2 = 100;

fn main() {
    //mutable / immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of const is: {}", CONST_X);

    //shadowing
    //https://en.wikipedia.org/wiki/Variable_shadowing
    //binding by 5
    let y = 5;
    //shadowing
    let y = y + 1;
    //shadowing
    let y = y * 2;
    //Error
    //y = y * 2
    println!("The value of y is: {}", y);
    
    //another type variable
    //5 white spaces
    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces in: {}", spaces);

    //Error
    //let mut spaces2 = "     ";
    //stringでimmutableな変数のstring->int型変換は不可能。shadowingなら可能。
    //spaces2 = spaces2.len();

    //data type
    //静的型付き言語なので、複数の型が推論される場合は型注釈が必須
    //この場合はparseによってintとstringが推論され、エラーとなってしまうためuint32(符号なし32bit整数)を指定
    let guess: u32 = "42".parse().expect("Not a Number!");
    println!("guess is: {}", guess);

    //type: scalar
    //4つ存在(int , float, bool, string)
    
    //type: int
    //今回はやっていないが、16進数表記や8進数表記、2進数表記も可能
    //Rustで整数が型推論された際の基準はi32となり、普通は処理速度も最速となる。
    //int8: -128~127
    let in8: i8 = -128;
    println!("The Value of int8 is: {}", in8);
    //uint8: 0~255
    let uin8: u8 = 255;
    println!("The Value of uint8 is: {}", uin8);
    //int16: -32768~32767
    let in16: i16 = -32768;
    println!("The Value of int16 is: {}", in16);
    //uint16: 0~65535
    let uin16: u16 = 65535;
    println!("The Value of uint16 is: {}", uin16);
    //int32: -2147483648~2147483647
    let in32: i32 = -2_147_483_648;
    println!("The Value of int32 is: {}", in32);
    //uint32: 0~4294967295?
    let uin32: u32 = 4_294_967_295;
    println!("The Value of uint32 is: {}", uin32);
    //int64: -9223372036854775808~9223372036854775807
    let in64: i64 = -9_223_372_036_854_775_808;
    println!("The Value of int64 is: {}", in64);
    //uint64: 0~18446744073709551615
    let uin64: u64 = 18_446_744_073_709_551_615;
    println!("The Value of uint is: {}", uin64);

    //type: float
    //floatの標準はf64
    let fl32: f32 = 10000.0;
    println!("The Value of f32 is: {}", fl32);
    //下のように型注釈なしの場合、float64
    let fl64 = 3.2;
    println!("The Value of f64 is: {}", fl64);

    //numeric operation
    //addition
    let sum = 5 + 10;
    println!("The Value of sum is: {}", sum);

    //diff
    let difference = 95.5 - 4.3;
    println!("The Value of difference is: {}", difference);

    //product
    let product = 4 * 30;
    println!("The Value of product is: {}", product);

    //quotient
    let quotient = 56.7 / 32.2;
    println!("The Value of quotient is: {}", quotient);

    //remainder
    let remainder = 43 % 5;
    println!("The Value of remainder is: {}", remainder);

    //type: boolean
    let t = true;
    println!("The Value of true is: {}", t);
    let f = false;
    println!("The Value of false is: {}", f);

    //type: character
    //unicode scalar
    let c = 'z';
    println!("The Value of character is: {}", c);

    //type: tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (uno, dos, tres) = tup;
    println!("The Value of tuple in 1 is: {}", uno);
    println!("The Value of tuple in 2 is: {}", dos);
    println!("The Value of tuple in 3 is: {}", tres);
    //添え字は0スタート
    println!("The Value of tuple in 3(.) is: {}", tup.2);

    //type: array
    //array内要素の型はすべて等しくなくてはならない
    let arr = [1,2,3,4,5];
    println!("The Value of array is: {}", arr[0]);
}

fn main() {
    //if else
    let number = 3;
    //ifの条件式は必ずboolでなくてはならない。
    if number < 5 {
        println!("condition was true");
    }else {
        println!("condition was false");
    }
    
    /*これは"mismatched types"のエラー
    if number {
        println!("number was three");
    }
    */

    if number != 0 {
        println!("number was something other than zero");
    }

    //else if
    //複数のelseifが存在する場合、最初にマッチした一つに入り、その後は評価されない。
    //また、複数if elseが存在する場合、matchを使うことがrustでは推奨されているよう。
    let number2 = 6;
    if number2 % 4 == 0 {
        println!("number is divisible by 4");
    }else if number2 % 3 == 0 {
        println!("number is divisible by 3");
    }else if number2 % 2 == 0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if in let
    //この場合、numberに入り得る値の型が間違っていた場合、エラーとなる。
    let condition = true;
    let number = if condition {
        5
    }else {
        //6.2や"6"等にした場合、上の枝でintが値として用意されているため、エラーとなる。型は同じでなくてはならない。
        6
    };

    println!("The Value of number is: {}", number);
}

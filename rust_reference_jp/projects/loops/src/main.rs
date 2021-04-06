fn main() {
    //無限ループ(breakで終了)
    loop {
        println!("again!");
        break;
    }
    
    //条件付きループ(while)
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //list check by while
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("(while) the value is: {}", a[index]);
        index = index + 1;
    }

    //list check by for
    let b = [10,20,30,40,50];
    //list bのイテレータ変換
    for element in b.iter() {
        println!("(for) the value is: {}", element);
    }

    //for range
    //range型は(x..y)の場合、xからy未満の数までの数値を順番に生成する。(イテレータ的な理解でいい？)
    //.rev()によって逆順になるため、3,2,1となる。
    for number in (1..4).rev() {
        println!("range countdown {}!", number);
    }
    println!("LIFTOFF!!!");
}

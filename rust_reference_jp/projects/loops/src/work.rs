fn main() {
    //bidirectional transform at fahrenheit and celsius
    let fah = cel_2_fah(14.0);
    println!("{}", fah);
    let cel = fah_2_cel(2.0);
    println!("{}", cel);

    //fibonacci
    //Fn
    let n = 12;
    let mut uno = 0;
    let mut dos = 1;
    let mut sum = 0;
    for number in 0..n {
        sum = uno + dos;
        if number % 2 == 0 {
            dos = sum;
        }else {
            uno = sum;
        }
    }
    println!("{}", sum);

    //Twelve Days of Christmas
    let words1 = "On the";
    let words2 = "day of Christmas my true love gave to me";
    for number in 1..13 {
        let day: String = match number {
            1 => "first".to_string(),
            2 => "second".to_string(),
            3 => "third".to_string(),
            4 => "fourth".to_string(),
            5 => "fifth".to_string(),
            6 => "sixth".to_string(),
            7 => "seventh".to_string(),
            8 => "eighth".to_string(),
            9 => "ninth".to_string(),
            10 => "tenth".to_string(),
            11 => "eleventh".to_string(),
            12 => "twelfth".to_string(),
            _ => "none".to_string(),
        };
        println!("{} {} {}", words1,day,words2);
        if number > 11 {
            print!("Twelve drummers drumming, ");
        }
        if number > 10 {
            println!("Eleven pipers piping, ");
        }
        if number > 9 {
            print!("Ten lords a leaping, ");
        }
        if number > 8 {
            println!("Nine ladies dancing, ");
        }
        if number > 7 {
            print!("Eight maids a milking, ");
        }
        if number > 6 {
            println!("Seven swans a swimming, ");
        }
        if number > 5 {
            print!("Six geese a laying, ");
        }
        if number > 4 {
            println!("Five gold rings, ");
        }
        if number > 3 {
            print!("Four calling birds, ");
        }
        if number > 2 {
            println!("Three French hens,");
        }
        if number > 1 {
            print!("Two turtle doves and a partridge in a pear tree ");
        }
        if number > 0 {
            println!("A partridge in a pear tree\n");
        }
    }
}

fn cel_2_fah(x: f64) -> f64{
    x * 1.8 + 32.0
}

fn fah_2_cel(x: f64) -> f64{
    let x = x - 32.0;
    x / 1.8

}

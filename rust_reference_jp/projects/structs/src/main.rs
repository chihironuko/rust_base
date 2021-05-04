//構造体は中のフィールドに関して所有権を持っていなくてはならないため、&strの様な参照を保持しない様にする。
//参照の保持も可能ではあるらしいので、需要には対応している。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//deriveつけると開発者用出力を行うことができる。トレイトとやらが関係あるらしいが。。。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//impl : implementation(実装)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//同じ名称のimplementationの存在
//同じ名称でどちらも使える。
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

//構造体は中のフィールドに関して、
fn build_user(email:String, username:String) -> User {
    User {
        //フィールド省略記法(変数とフィールド名が同一の時に省略可能)
        email,
        username,
        sign_in_count: 1,
        active:true,
    }
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    //instanse has element "mutable/immutable"
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //value reference is dot-notation
    println!("{}", user1.username);

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    //create struct by function
    let user3 = build_user(String::from("username"), String::from("email"));
    println!("{}", user3.email);


    let user2 = User {
        email: String::from("anotherecample@email.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    //タプル構造体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //rectangle
    let height1 = 30;
    let width1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //tuple rectangle
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {width:30, height: 50};
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    //derive注釈をつけた構造体の出力指定子色々あるアルヨ
    println!("rect is {:?}", rect2);
    println!("rect is {:#?}", rect2);

    let rect3 = Rectangle{width: 30, height: 50};
    println!(
        "The area of Rectangle is {} square pixels",
        rect3.area()
    );

    //別々のインスタンスの持つ値を比較
    let rect4 = Rectangle{width:30, height:50};
    let rect5 = Rectangle{width:10, height:40};
    let rect6 = Rectangle{width:60, height:45};

    //参照渡しなのは、所有権を関数側に移した場合に、関数の終わりでインスタンスに死なれたら困るため。
    println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));
    println!("Can rect4 hold rect6? {}", rect4.can_hold(&rect6));

    //関連関数の作成
    let sq = Rectangle::square(12);
    println!("sq is {:#?}", sq);
}

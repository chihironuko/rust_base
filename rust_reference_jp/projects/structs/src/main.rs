//構造体は中のフィールドに関して所有権を持っていなくてはならないため、&strの様な参照を保持しない様にする。
//参照の保持も可能ではあるらしいので、需要には対応している。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//構造体は中のフィールドに蟹sて、
fn build_user(email:String, username:String) -> User {
    User {
        //フィールド省略記法(変数とフィールド名が同一の時に省略可能)
        email,
        username,
        sign_in_count: 1,
        active:true,
    }
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

    let user2 = User {
        email: String::from("anotherecample@email.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    //タプル構造体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let Origin = Point(0, 0, 0);
}

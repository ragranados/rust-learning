#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

struct Color(i32, i32, i32);

fn main() {
    // let mut user = build_user(
    //     String::from("myusername"),
    //     String::from("myemail@email.com"),
    // );

    // println!("Valor: {}", user.email);

    // user.email = String::from("mynewemail@email.com");

    // println!("Valor: {}", user.email);

    // let user2 = User {
    //     username: String::from("newusername"),
    //     ..user
    // };

    // println!("Valor: {}", user.username);

    // let red = Color(0, 0, 0);

    // let Color(red, blue, green) = red;

    // println!("Red: {red}, Blue: {blue}, Green: {green}");

    // --- Calculo de area de rectangulo ---
}



fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_account: 1,
    }
}

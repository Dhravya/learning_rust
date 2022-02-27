#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = build_user(String::from("dhravyashah@gmail.com"), String::from("dhravyashah"));
    println!("{:?}",  user1);

    let user2 = build_user(user1.email, String::from("Hello, world"));
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("dhravyacodeshah@gmail.com"),
        sign_in_count: 30,
        ..user2
    };
    println!("{:?}", user3);

    // ! Doesn't work
    // println!("{:?}", user2)

    struct AlwaysEqual;
    let thing = AlwaysEqual;
    
}

fn build_user(email: String, username: String) -> User {

    // this also works
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }


}

// Instantiating a struct.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user name:{}", user1.username);
    println!("user email:{}", user1.email);

    user1.email = String::from("email@email.com");

    println!("user email:{}", user1.email);

    let user2: User = build_user(String::from("strong@strong.com"), String::from("username2"));

    println!("User two email: {}", user2.email);

    let user3 = User {
        email: String::from("weaker@stronger.com"),
        ..user1
    };
    
    println!(
        "User3 detalis: email = {}, username = {}, active = {}, sign_in_count = {}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    )
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

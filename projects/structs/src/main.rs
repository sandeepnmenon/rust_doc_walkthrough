struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = build_user(
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1 // user1 cannot be used after this as = moves the data
    };
    // If we had given new values for email and username for user2, then we could hvae used user1
    // Why? Because the remaining fields active and sign_in_count both implement the Copy trait

    // Tuple structs. Useful when we need to give a tuple a name
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color black ({},{},{})", black.0, black.1, black.2);
    println!("Origin  ({},{},{})", origin.0, origin.1, origin.2);

    // Unit-Like Structs without any Fields. Useful when you need to implement a trait on some type but dont have any data to be stored
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sing_in_count: 1,
    // }

    // If parameter name and struct field name are same the below field init shorthand can be used
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

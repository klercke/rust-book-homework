struct User { // this is a struct called User
    active: bool, // this is a field called active of type bool
    username: String, // each instance of this struct will own its fields, since they are not references
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User { // this is a function that returns a User struct
    User { // this is a User struct instance, which is impilictly returned by the function
        active: true, // newly created users are active by default
        username, // this is a shorthand for username: username. it works because the field and the variable have the same name
        email, // same as above, but for the email field
        sign_in_count: 1, // account creation counts as a sign in
    }
}

fn print_user_info(user: User) { // we can take in the user struct as a parameter
    println!("information for {}:", user.username);
    println!("email: {}", user.email);
    println!("active: {}", user.active);
    println!("sign in count: {}\n", user.sign_in_count);
}

struct Color(i32, i32, i32); // this is a tuple struct called Color
                             // it works like a struct, but the fields don't have names, just types
struct Point(i32, i32, i32); // this is another tuple struct called Point

struct AlwaysEqual; // this is a unit struct called AlwaysEqual
                    // it works like a struct, but it has no fields
                    // this can be useful for implementing traits on types that don't have any data to store

fn main() {
    let user1 = User {
        active: true, // the order of the fields doesn't matter
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("information for {}:", user1.username); // we can access information with dot notation
    println!("email: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign in count: {}\n", user1.sign_in_count);

    let mut user2 = User { // we can make a mutable instance of a struct. the entire instance must be mutable, not just some fields
        active: true,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
    };

    user2.sign_in_count = 2; // we can change the values of the fields if the struct instance is mutable

    println!("information for {}:", user2.username);
    println!("email: {}", user2.email);
    println!("active: {}", user2.active);
    println!("sign in count: {}\n", user2.sign_in_count);

    let username = String::from("user3");
    let email = String::from("user3@example.com");
    let user3 = build_user(email, username); // we can use a function to create a struct instance
    print_user_info(user3); // we can use a function to print the information of a user, since we will be doing it multiple times and already know the names of the fields

    let user4 = User {
        active: user1.active, // we can use the values of another struct instance to create a new one
                              // but doing this for a field that is of a type that does not have the Copy trait, like String, will move the value
                              // which will then invalidate the first struct instance
        username: String::from("user4"),
        email: String::from("user4@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user5 = User {
        username: String::from("user5"), // if we manually specify the things that should be different,
        email: String::from("user@example.com"),
        ..user4 // then we can use the rest of the values from another struct instance to create a new one
                // this can still invalidate the first struct instance if one of the remaining fields is of a type that does not have the Copy trait
    };

    let black = Color(0, 0, 0); // we can create an instance of a tuple struct like this
    let origin = Point(0, 0, 0); // even though the fields are the same, these are different types, so we can't add a Color to a Point, for example

    println!("black: (R{}, G{}, B{})", black.0, black.1, black.2); // we can access the fields of a tuple struct instance with dot notation

    let subject = AlwaysEqual; // we can create an instance of a unit struct like this
}

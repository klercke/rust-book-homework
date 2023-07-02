enum IpAddrKind { // since there are a specific number of IP protocol versions, v4 and v6, we can create a enum to represent them
    v4(String), // by adding the data type to each variant, we can store associated data with each variant
    v6(String), // in this case, it works almost like a primitive struct
}

fn route(ip_kind: IpAddrKind) { // we can then use this enum in a function
    // code would go here
}

// we can also make an enum that has different types of associated data
enum IpAddrType {
    v4(u8, u8, u8, u8),
    v6(String),
}

// the standard library has a similar enum, called IpAddr, that is used to store IP addresses. it is defined like this:
struct Ipv4Addr {
    // code here
}
struct Ipv6Addr {
    // code here
}
enum IpAddr { // we can define this because we have not brought the standard library's IpAddr into scope
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// another example of an enum with different types of associated data
enum Message {
    Quit, // this variant has no data associated with it
    Move { x: i32, y: i32 }, // this variant has fields like a struct
    Write(String), // this variant has a single String associated with it
    ChangeColor(i32, i32, i32), // this variant has three i32 values associated with it
}

// the alternative to an enum is to define different structs for each variant:
// the issue with doing this is that we can't easily define a function that can take multiple of these structs as an argument
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// we can also define methods on enums
impl Message {
    fn call(&self) {
        // code here
    }
}

// the Option enum is another example of an enum defined in the standard library
// the T syntax is a generic type parameter, which means that the Some variant of the Option enum can hold one piece of data of any type
// enum Option<T> {
//     None,
//     Some(T),
// }
// the only way to have a value of None is to explicitly opt into it
// so if we have a type of anything other than Option<T>, we can safely assume that the value is not None
// handling an Option<T> value is a common operation, so there are a number of methods defined on it
// generally, we want to have code that only runs if we have a Some(T) value, and allow that code to use the inner T
// we would then have other code that runs if we have a None value, and that code would not need to use the inner T
// this is where the match expression comes in

fn main() {
    let ipv4 = IpAddrKind::v4(String::from("127.0.0.1")); // we can create instances of each of the two variants of IpAddrKind like this
    let ipv6 = IpAddrKind::v6(String::from("::1")); // these are have the same type, IpAddrKind, and can be used in the same ways

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5); // the Some variant of the Option enum is used when there is a value
                                            // the implied type of this variable is Option<i32>
    let some_char = Some('a'); // this variable has an implied type of Option<char>
    let absent_number: Option<i32> = None; // the None variant of the Option enum is used when there is no value
                                           // because there is no value, we must explicitly define the type of this variable
                                    

}

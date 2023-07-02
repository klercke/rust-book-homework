fn main() {
    // s is not valid here, it’s not yet declared
    {
        let mut s = String::from("hello");  // s is valid from this point forward

        s.push_str(", world!");   // push_str() appends a literal to a String

        println!("{}", s); // this will print 'hello, world!'
    } // this scope is now over, and s is no longer valid
    // s is automatically freed by Rust calling the drop function of the String type

    let x = 5;
    let y = x; // x is copied into y. Both x and y are variables on the stack with a value of 5
                    // we can create a copy because we know the size of x and y at compile time, so they live on the stack
                    // this means creating a copy is computationally inexpensive and can be done automagically


    let s1 = String::from("hello");
    let s2 = s1; // the length, capacity, and pointer to the memory on the heap are copied from s1 to s2
                         // this means that s1 and s2 both have values on the stack that point to the same location on the heap
                         // Rust considers s1 to no longer be valid and therefore does not have to worry about freeing the memory when s1 goes out of scope
                         // because this makes a shallow copy and then invalidates the original, it is called a "move". s1 was moved into s2
                         // since deep copies are not created automatically, automatic copies can always be assumed to be computationally inexpensive

    let s3 = String::from("hello");
    let s4 = s3.clone(); // this creates a deep copy of the data on the heap
                                 // this is an explicit way to copy the heap data of one String into another
                                 // this is more expensive than a move, so it is not automatically done


    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value is moved into the function
                                    // so s is no longer valid on this line


    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function, but since x is an i32, it is copied into the function instead
                                // so x is still valid on this line


    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3


    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1 is moved into calculate_length, which gives calculate_length ownership of s1
                                                         // so calculate_length needs to return s1 back to us (as s2 in this case) so main can still use it
    println!("The length of '{s2}' is {len}.");


    let s1 = String::from("hello");
    let len = calculate_length_with_reference(&s1); // s1 is passed by reference to calculate_length_with_reference
                                                           // so main still owns s1 and we do not need to return it from calculate_length_with_reference
    println!("The length of '{s1}' is {len}.");


    let mut s = String::from("hello");
    change(&mut s); // since we pass a mutabale reference to s, which is itself a mutable string, we can modify it in the function
                                // if a piece of data has a mutable reference, then no other references to that data can be used until the mutable reference goes out of scope
                                // so trying to create another reference to s would fail
                                // this prevents data races
    println!("{s}"); 
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and drop is called. the memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. x is still usable

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
} // some_string goes out of scope, but its value has been returned and stored in s1

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
} // a_string goes out of scope, but its value has been returned and stored in s3

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // we need to return both the String and its length, else the String would be dropped
}

fn calculate_length_with_reference(s: &String) -> usize { // s is a reference to a string, rather than a string itself
                                                          // since we are only borrowing the value of s, we cannot change it in this function
    s.len()
} // s goes out of scope, but since it does not have ownership of what it refers to, nothing happens

fn change(some_string: &mut String) { // some_string is a mutable reference to a string, so it will not be dropped and we can modify it
    some_string.push_str(", world");
}
fn first_word(s: &String) -> usize { // returns a usize which is the index of the end of the first word in a string
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is a byte literal, and since item is a byte, if item is b' ', then it is a space
            return i;
        }
    }

    s.len() - 1 // if we don't find a space, then the first word is the whole string, so we just return the last index
}

fn first_word_slice(s: &String) -> &str { // &str is the type representing a string slice
    // we will do the same thing as above, but instead of returning a usize, we will return a string slice containing the first word
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_improved(s: &str) -> &str { // this is exactly the same as first_word_slice, but it accepts a string slice instead of a String
                                                // since you can always take a slice of a string, this is more flexible
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // this works, but is not ideal
    let mut s = String::from("hello world"); // s needs to be mutable to clear it later
    let word = first_word(&s); // word will get the value 5
    let first_word = &s[..word];
    println!("The first word is {first_word}");
    s.clear(); // this empties the String, making it equal to ""

    // word is still set to 5 here, but s is now an empty string
    // this would cause unexpected results if we tried to use word later

    let s = String::from("hello");
    // you can omit the 0 at the beginning of a range
    // you can also omit the number at the end of a range if it is equivilant to the last index of the String
    // so the following statements are all equivalent for our string above
    // slices must occur at the boundaries of UTF-8 characters, so creating a slice in the middle of a multi-byte character will cause an error
    let slice = &s[0..5];
    let slice = &s[..5];
    let slice = &s[..=4];
    let slice = &s[..(s.len())];
    let slice = &s[0..];
    let slice = &s[..];

    // this is the preferred way to get the first word of a string
    // it's safer and easier to read
    let s: String = String::from("hello world");
    let first_word = first_word_slice(&s);
    println!("The first word is {first_word}");

    let s = "Hello, world!"; // s is a string literal, which is a slice pointing to a specific point in the binary

    let s: String = String::from("hello world");
    let first_word = first_word_slice_improved(&s[..]); // we convert s to a string slice, then pass it to first_word_slice_improved
                                                              // this version of first_word is the safest and most flexible
                                                              // we could also pass a string literal to first_word_slice_improved directly,
                                                              // since a string literal is already a slice
    println!("The first word is {first_word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // this is a slice of the array a, and it has the type &[i32]
    assert_eq!(slice, &[2, 3]); // the two are necessarily equal
}

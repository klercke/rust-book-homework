// implements a library representing a restaurant
// the functions are just stubs as the idea is to show how to structure a library, not simulate a restaurant
// the modules serve to group functions that are related to each other
// this makes it easier to find functions and to understand the code
// we have this module tree now:
// crate
// └── front_of_house
// ├── hosting
// │   ├── add_to_waitlist
// │   └── seat_at_table
// └── serving
//     ├── take_order
//     ├── serve_order
//     └── take_payment

pub mod front_of_house; // this is implemented in src\front_of_house.rs

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super goes up one level in the module tree
                                // this is useful when the calling code is unlikely to be separated from its parent module, but the parent module may be moved in the module tree
    }

    fn cook_order() {}

    // structs and enums can also be made public
    pub struct Breakfast {
        pub toast: String, // this field will be public
        seasonal_fruit: String, // this one will be private
                                // this is emulating the idea that the customer can pick the toast, but the fruit is seasonal and the restaurant decides what it is
    }

    impl Breakfast {
        // we need to implement a public constructor, otherwise we would not be able to create a Breakfast outside of the module since seasonal_fruit is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), // this is a private field, but it can be accessed from inside the module
                                                         // the "chef" still gets to decide what the fruit is, but the customer can't pick it
            }
        }
    }

    // making an enum public makes all of its variants public as well
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// this is a public function because we want it to be able to be called from outside the library
pub fn eat_at_restaurant() {
    // absolute path
    // will remain valid if eat_at_restaurant is moved to another module
    // absolute paths are preferred because they are more likely to remain valid when refactoring
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // will remain valid if front_of_house and eat_at_restaurant are moved to another module
    front_of_house::hosting::add_to_waitlist(); 

    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // change our mind about what bread we want
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // this won't compile if uncommented because seasonal_fruit is private
    // we aren't allowed to see or modify what fruit comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // we can use the variants of Appetizer here because it is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // if we "import" the front_of_house module with use, we can use its functions without the full path
    // this works like a symlink in a filesystem, but only for the scope in which use was called
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // we could also use just the function, but this is less idiomatic and can cause confusion, so it is not recommended
    use crate::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    // with structs, enums, or other items, the idiomatic way is to use the full path:
    // this isn't restaurant related, but whatever. neither are the next two examples
    use std::collections::HashMap;
    let mut map = HashMap::new(); // this is much clearner than writing std::collections::HashMap or collections::HashMap each time
    map.insert(1, 2);

    // we can't use the same name for two items in the same scope, so we need to use their parents:
    use std::fmt;   // if we used use std::fmt::Result, we would not be able to use use std::io::Result, so this is the more idiomatic solution
    use std::io;
    fn function1() -> fmt::Result {Ok(())}
    fn function2() -> io::Result<()> {Ok(())}

    // we can also use "use as" to rename items:
    use std::fmt::Result;
    use std::io::Result as IoResult; // since we already imported Result, we need to rename this to avoid a conflict
    fn function3() -> Result {Ok(())}
    fn function4() -> IoResult<()> {Ok(())} // and it works just fine

    // items brought into scope with use are private by default, so we can't use them outside of the scope in which use was called
    // we can fix this with pub use
    // this is useful when the code is structured in a way that might conflict with the intuitive way to use the library
    // in the restaurant example, this lets us program the front of house and back of house as separate modules, but lets our "customers" use them as if they were one module, the restaurant
    pub use crate::front_of_house::hosting as pub_hosting; // I had to rename this because otherwise it would conflict with the hosting module I imported in an earlier example

    // we can also use nested paths to bring multiple items into scope with less code
    use std::{cmp::Ordering, arch}; // this is the same as "use std::cmp::Ordering; use std::arch;"

    // nested paths can be used at any level
    use std::env::{Args, Vars}; // this is the same as "use std::env::Args; use std::env::Vars;"

    // finally, if we want to bring all public items in a path into scope, we can use glob operator
    // or as I like to call it, the globerator
    use std::collections::*; // this will bring all public items into scope. this should be used with caution, because it can cause name conflicts, 
                             // make things hard to read, and make it unclear where a name is defined


}
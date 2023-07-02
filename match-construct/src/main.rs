enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state
enum UsState { // enum of all 50 states so the quarter variant can store a state
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { // we can add code to each arm of a match expression
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5, // or we can just return a value
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); // we can use the state variable to do something
            25
        },   
    }
}

// we can write functions that match on Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // we can match on Option<T> and return None if the value is None
                      // the compiler will throw an error if we don't handle all cases, so we must handle None
        Some(i) => Some(i + 1), // or we can match on Some(i) and return Some(i + 1)
    }
}

fn main() {
    let coin1 = Coin::Penny;
    println!("The value of coin1 is {}", value_in_cents(coin1));

    let coin2 = Coin::Quarter(UsState::Wyoming);
    println!("The value of coin2 is {}", value_in_cents(coin2));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of five is {:?}", five);
    println!("The value of six is {:?}", six);
    println!("The value of none is {:?}", none);

    // we can choose to match only some cases and use other to match all other cases:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // if we didn't need the value of the other cases, we could use _ instead:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // the compiler will not warn us about not using the value of the "other" case if we use _
    }

    // we could also choose for nothing to open in the "other" case:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // the unit value works like pass in Python, so this is just saying "in any other case, do nothing"
    }

    
}

// these four functions are just here so the match expression above compiles
fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

fn move_player(spaces: u8) {
    println!("Moving player {} spaces!", spaces);
}

fn reroll() {
    println!("Rerolling!");
}
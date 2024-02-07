
// Examples of the "match" control flow construct:

fn main() {
    let coin = Coin::Nickel;
    println!("The value of the coin is: {}", value_in_cents(&coin));

    // Multiple layers, since the Coin has a nested property
    value_in_cents(&Coin::Quarter(UsState::Arizona));

    // Examples of handling null values
    let five: Option<i32> = Some(5);
    let none: Option<i32> = None;
    plus_one(five); // returns 6
    plus_one(none); // returns None

    match coin {
        Coin::Penny => println!("Penny!"),
        Coin::Nickel => println!("Nickel!"),
        other => println!("Not a penny or nickel! It's a {:?}", &other), // uses a custom variable to handle edge cases
        _ => println!("No idea what this is.") // catch-all pattern, when you don't want to use the variable
        // Unreachable, since 'other' catches everything before '_' is reached.
    }

    // Another example of catch-all arms:
    let dice_roll = 9;
    match &dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(*other), // uses the dice_roll value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // Doesn't use the dice_roll value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Doesn't do anything if not 3 or 7
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    // Inputting all of the US states has been left as an exercise for the reader
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // quarters have a nested enum
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // We have to define a handler for the 'state' of the quarter
            println!("This quarter was printed in the state of {:?}", state);
            25
        }
    }
}

// An example of a function that uses Options to avoid null value issues
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // Note that ALL cases must be addressed. For Options, that's two cases:
        None => None, // The null case
        Some(i) => Some(i + 1), // All other cases
    }
}

// Some empty functions for examples
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn move_player(distance: u8) {}
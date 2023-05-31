// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// * Use an enum to create different flavors of drinks
enum Flavor {
    Lemon,
    Cherry,
    Coke
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {   
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Lemon => println!("flavor: Lemon"),
        Flavor::Cherry => println!("flavor: Cherry"),
        Flavor::Coke => println!("flavor: Coke"),
    }

    println!("oz: {:?}", drink.fluid_oz);

fn main() {
    let lemon = Drink {
        flavor: Flavor:: Lemon,
        fluid_oz: 6.0
    };
    print_drink(lemon);
    let Coke = Drink {
        flavor: Flavor:: Coke,
        fluid_oz: 6.0
    };
    print_drink(coke);
}

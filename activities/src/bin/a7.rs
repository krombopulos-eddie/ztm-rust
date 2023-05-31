// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Blue,
    Yellow,
    White,
    Black,
    Red
}

fn print_color(my_color: Color){
    match my_color {
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yello"),
        Color::White => println!("white"),
        Color::Black => println!("black"),
        Color::Red => println!("red"), 
    }
}


fn main() { 
    print_color(Color::Black);
}

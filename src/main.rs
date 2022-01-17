use std::io;

//use std::iter;
//use rand::Rng;

fn main() {
    println!("Enter the numbers of elements to convert into Romanian numbers");
    let user_input = enter_value();
    let converter_int: usize = string_to_integer(user_input);
    //fizz_operation(converter_int);
    to_rom_nummb(converter_int);
}


fn enter_value() -> String {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed read the line.");
    value
}

fn string_to_integer(str_to_int: String) -> usize {
    let number: usize = str_to_int.trim().parse().expect("Please, enter the unsigned number");
    number
}

fn fizz_operation(number: u32) {
    for i in 1..number {
        match (i % 2, i % 3) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("Just {}", i),
        }
    }
    println!("Job is done!");
}

fn to_rom_nummb(amount: usize) -> String {
    let basic_roman_num = "I".repeat(amount);
    // println!("Romanian number is: {}", BasicRomanNum);
    //BasicRomanNum
    let roman_numb = str::
    replace(&basic_roman_num, "IIIII", "V")
        .replace("VV", "X")
        .replace("XXXXX", "L")
        .replace("LL", "C")
        .replace("CCCCC", "D")
        .replace("DD", "M")
        ;
    println!("Romanian number is: {}", roman_numb);
    roman_numb
}

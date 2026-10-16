use std::io; // this tells that we are using standard library to tale input
fn main() {
    println!("Enter your name");
    let mut name = String::new(); // String::new() is used to take input & it should be mutable so that we can actually assign a value to it
    io::stdin() // This is a process in which the string value we entered is regestere to variable
        .read_line(&mut name) // it checks thst what data has been inputed
        .expect("Faled to take input"); // it insted of having a error will show a message if the user inputed a non string value
    println!("hi {}", name);
    // but there is a problem with it it can only take string for that we have another ways
    println!("Enter your age :");
    let mut age = String::new(); // it will take input
    io::stdin() // this process will remain same
        .read_line(&mut age)
        .expect("Failed to read the line");
    //now we will convert string to int
    let age: i32 = age.trim().parse().expect("please enter a valid number");
    // in this line actually didn't convert the data type of age we are just shadowing
    // .trim() is because when you add a number in the input the system automatically adds something like this "16\n" with that number
    // and to remove this we use .trim()
    // .prase() is for the conversion of string into integer
    // .expect is used incase the input is invalid
    // and one more thing u32 is also a data type almost like str but it does not include negative numbers & it is called unsigned integer
    // and we need to tell wich datatype it should convert into it can be float integer or unsigned integer and even for bool
    // in case of a different datatype it will give error
    println!("You were {} years old last year", age - 1);
}

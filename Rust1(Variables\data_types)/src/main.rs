fn main() {
    let name = "Anurag"; //it's a string that can't be changed
    let friend: &str = "no one :("; //is also a string but is defined defining is optional in rust
    let mut age = 16; //it is a variable that can be changed as we have defined by adding mut in front of the variable
    age += 1; // is going to happen in september
    let mut income = 0.00; // hope it to change :)> and it is a flost value
    let currency: char = '$'; // it is a single character conataining variable
    let survival = true; //or may be not :0
    println!(
        "i am {} my friends are {} my age is {} and my income is {}{} and i am alive {}",
        name, friend, age, currency, income, survival
    );
} // the println!() print's a new line and print!() just print's "{}" this symbol eis constantly used in the print
// statement cause this symbol tells that were to print/ place the variable value

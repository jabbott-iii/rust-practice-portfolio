use std::io;
use std::io::BufRead;
// f64 is a data type that represents a 64-bit floating point number

fn main() {
    println!("Tip Calculator");
    
    println!("What is the total bill? $");
    let mut bill = String::new(); // Variable to hold the total bill amount
    io::stdin() // Get standard input
        .read_line(&mut bill) // Read a line into the bill variable
        .expect("Failed to read line"); // Handle potential errors
    let decimal_bill: f64 = bill
        .trim_end() // Remove any trailing newline characters
        .parse() // Parse the string to a floating-point number
        .expect("Please enter a valid number"); // Handle potential parsing errors

    println!("What percentage would you like to give as a tip? ");
    let mut tip = String::new(); // Variable to hold the tip percentage
    io::stdin() // Get standard input
        .read_line(&mut tip) // Read a line into the tip variable
        .expect("Failed to read line"); // Handle potential errors
    let decimal_tip: f64 = tip
        .trim_end() // Remove any trailing newline characters
        .parse() // Parse the string to a floating-point number
        .expect("Please enter a valid number"); // Handle potential parsing errors

    let bill = decimal_bill; // Total bill amount as f64
    let tip = decimal_tip; // Tip percentage as f64

    let percent = tip / 100.0; // Calculate the tip percentage
    let total = percent * bill; // Calculate the total tip amount

    println!("How many people are you splitting the bill with? Including yourself. ");
    let mut friends = String::new(); // Variable to hold the number of people
    io::stdin() // Get standard input
        .read_line(&mut friends) // Read a line into the friends variable
        .expect("Failed to read line"); // Handle potential errors
    let decimal_friends: f64 = friends
        .trim_end() // Remove any trailing newline characters
        .parse() // Parse the string to a floating-point number
        .expect("Please enter a valid number"); // Handle potential parsing errors

    let friends = decimal_friends; // Number of people as f64

    let final_amount = total / friends; // Calculate the individual share of the tip

    println!("Individual share of the tip is ${:.2}", final_amount); // Display the individual share rounded to 2 decimal places

    println!("Program finished. Press Enter to exit...");
    let stdin = io::stdin(); // Get standard input
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}
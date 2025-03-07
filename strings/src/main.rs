/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.

Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.

Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.

Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]

Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.

Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/

use std::io;

fn make_money(value: &mut String) {
    value.push_str("SSS")
}

fn trim_and_capitalize(value: &str) -> String {
    value.replace(" ", "").to_string().to_uppercase()
}

fn elements(value: &str) -> Vec<&str> {
    value.split("!").collect()
}

fn get_identity() -> String {
    let mut name = String::new();
    println!("Insert your First name");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to obtain the First Name");

    let mut last_name = String::new();
    println!("Insert your Last name");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to obtain the First Name");

    format!("{} {}", name.trim(), last_name.trim())
}

fn main() {
    let mut value = String::from("Hoy!Si!Ayuno");
    make_money(&mut value);

    println!("{value:?}");

    println!("{:?}", trim_and_capitalize(value.as_str()));

    println!("{:?}", elements(value.as_str()));

    println!("{}", get_identity());
}

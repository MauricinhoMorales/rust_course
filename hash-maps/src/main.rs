use std::collections::HashMap;

/*
Bring the HashMap type into the current's file's namespace.

Declare a `sauces_to_meals` HashMap. The keys will be
string slices and the values will be a vector of string
slices. Use the `from` function to populate the HashMap
with 2 key-value pairs:

Key: "Ketchup"
Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]

Key: "Mayonnaise"
Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]

Use the `insert` method to add the following key-value
pair to the HashMap.

Key: "Mustard"
Value: Vector of ["Hot dog", "Burgers", "Pretzels"]

Use the `remove` method to remove the key-value pair
where "Mayonnaise" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `get` method to retrieve the key-value pair
where "Mustard" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `entry` and `or_insert` methods to add the
following key-value pair:

Key: "Soy Sauce"
Value: Vector of ["Sushi", "Dumplings"]

Finally, print out the final `sauces_to_meals` HashMap.

The final result should be:
{
  "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
  "Soy Sauce": ["Sushi", "Dumplings"],
  "Mustard": ["Hot dog", "Burgers", "Pretzels"]
}
*/

fn main() {
    let data = [
        (
            "Ketchup",
            Vec::from(["French Fries", "Burgers", "Hot Dogs"]),
        ),
        (
            "Mayonnaise",
            Vec::from(["Sandwiches", "Burgers", "Coleslaw"]),
        ),
    ];
    let mut sauces_to_meals = HashMap::<&str, Vec<&str>>::from(data);

    sauces_to_meals.insert("Mustard", Vec::from(["Hot dog", "Burgers", "Pretzels"]));

    match sauces_to_meals.remove("Mayonnaise") {
        Some(vector) => println!("{:?}", vector),
        None => println!("There was an error"),
    }

    match sauces_to_meals.get("Mustard") {
        Some(value) => println!("{:?}", value),
        None => println!("There was an error"),
    };

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{sauces_to_meals:#?}");
}

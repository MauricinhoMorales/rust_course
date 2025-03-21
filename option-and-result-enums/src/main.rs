/*
Define a Food struct with a single `name` field
set to a String. Derive a Debug implementation.

Define a Restaurant struct with a `reservations` field
set to a u32 and a `has_mice_infestation` field set to
a bool. Derive a Debug implementation.

Define a `chef_special` method on the Restaurant.
The method will return the restaurant's famous
dish. It should return an Option containing a Food
struct.

If the restaurant has a mice infestation, return the
None variant. There is no chef special!

If the restaurant has less than 12 reservations, return
a Food instance with a name of "Uni Sashimi" wrapped in
the Some variant. If it has 12 or more reservations,
return a Food instance with a name of "Strip Steak"
instead, also wrapped in the Some variant.

Define a `deliver_burger` method on the Restaurant.
It should accept an `address` string slice; it will
represent where to deliver the order. It should
return a Result type where the Ok variant holds a Food
struct and the Err variant holds a String.

If the restaurant has a mice infestation, return the
Err variant containing a String of "Sorry, we have a
mice problem".

If the user's address is an empty string, return the Err
variant with a String of "No delivery address specified".
HINT: You can use the `is_empty` method on a string to check
if it has 0 characters.
https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty

Otherwise, the delivery is good to go! Return the Ok
variant containing a Food struct with a `name` of "Burger".

In the `main` function, create a `Restaurant` instance
with 11 reservations and a mice infestation.

Invoke the `chef_special` method and print out its return
value. It should be the None variant.

Invoke the `deliver_burger` method with an argument of "123
Elm Street" and print out its return value. It should be
the Err variant.

Create another `Restaurant` instance with 15 reservations
and no mice infestation.

Invoke the `chef_special` method and print out its return
 value. It should be the Some variant with a "Strip Steak".

Invoke the `deliver_burger` method with an argument of an
empty address. Print out its return value. It should be the
Err variant.

Invoke the `deliver_burger` method again with an argument
of a valid address. Print out its return value. It should
be the Ok variant nesting a Food struct with a `name` of
"Burger".
*/
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            None
        } else {
            if self.reservations < 12 {
                Some(Food {
                    name: String::from("Uni Sashimi"),
                })
            } else {
                Some(Food {
                    name: String::from("Strip Steak"),
                })
            }
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err(String::from(
                "Sorry, we have a
mice problem",
            ))
        } else {
            if address.is_empty() {
                Err(String::from("No delivery address specified"))
            } else {
                Ok(Food {
                    name: String::from("Burger"),
                })
            }
        }
    }
}

fn main() {
    let restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", restaurant.chef_special());
    println!("{:?}", restaurant.deliver_burger("123 Elm Street"));

    let restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", restaurant.chef_special());
    println!("{:?}", restaurant.deliver_burger(""));
    println!("{:?}", restaurant.deliver_burger("NewYork"));
}

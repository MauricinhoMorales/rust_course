/*

THIS IS A TOUGH ONE.

The boss would like to see a vector of Customer
structs. Each Customer struct will hold the user's
id and a vector of their orders. Find a way to merge
the customer orders with the customers who made them,
then aggregate the data into Customer structs,
then collect the Customers in a vector, then
sort the collection by customer id.

The resulting vector should look like this:

[

Customer {
  id: 1,
  orders: [
    CustomerOrder { product: Microwave, quantity: 1, shipped: true },
    CustomerOrder { product: Fridge, quantity: 10, shipped: false }
  ]
},

Customer {
  id: 2,
  orders: [
   CustomerOrder { product: Blender, quantity: 3, shipped: false },
   CustomerOrder { product: Toaster, quantity: 2, shipped: false }
  ]
},

Customer {
  id: 3,
  orders: [
   CustomerOrder { product: Microwave, quantity: 5, shipped: true }
  ]
},

Customer {
  id: 4,
  orders: [
    CustomerOrder { product: Blender, quantity: 1, shipped: false }
  ]
}

]
*/

#![allow(unused, dead_code)]

use std::{cmp::Ordering, collections::HashMap, env};
#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    //  First

    let result = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();

    println!("Orders with Blenders: {:#?}", result);

    // Second

    let result = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Number of Microwaves ordered: {:#?}", result);

    // Third

    let arguments = env::args();

    let value = arguments.skip(1).take(1).fold(2, |init, argument| {
        if let Ok(value) = argument.parse::<u32>() {
            value
        } else {
            init
        }
    });

    let result = orders
        .iter()
        .filter(|order| order.quantity >= value)
        .collect::<Vec<&CustomerOrder>>();

    println!("Orders with atleast {value} quantity: {:#?}", result);

    // Fourth

    let mut hash = HashMap::<&Product, u32>::new();

    let result = orders.iter().fold(hash, |mut hash, order| {
        if !order.shipped {
            let value = hash.entry(&order.product).or_insert(0);
            let sum = *value + order.quantity;
            hash.insert(&order.product, sum);
        }
        hash
    });

    println!("HashMap {:#?}", result);

    // Fifth

    let mut result = orders.iter_mut().find(|order| !order.shipped);
    if let Some(mut order) = result {
        order.shipped = true;
    }

    println!("Customer orders: {:#?}", orders);

    // Sixth

    let mut result = orders.into_iter().zip(customer_ids_by_order.iter()).fold(
        Vec::<Customer>::new(),
        |mut vector, (order, customer_id)| {
            let opt_index = vector.iter().position(|item| item.id == *customer_id);
            if let Some(index) = opt_index {
                let update = vector.iter_mut().nth(index).unwrap();
                update.orders.push(order);
            } else {
                let mut orders = Vec::new();
                orders.push(order);
                vector.push(Customer {
                    id: *customer_id,
                    orders,
                })
            }
            vector
        },
    );
    result.sort_by_key(|customer| customer.id);

    println!("Customers: {:#?}", result);
}

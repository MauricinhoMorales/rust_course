#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}
#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        for mut item in &mut self.items {
            operation(&mut item)
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(Self),
    {
        operation(self)
    }
}

fn main() {
    let mut shopping_cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: String::from("APPLE"),
                price: 3.99,
            },
            SupermarketItem {
                name: String::from("BANANA"),
                price: 2.99,
            },
        ],
    };

    let closure = |item: &mut SupermarketItem| {
        item.price *= 0.85;
    };
    shopping_cart.traverse_items(closure);
    println!("{:?}", shopping_cart);

    let closure = |item: &mut SupermarketItem| {
        item.name = item.name.to_lowercase();
    };
    shopping_cart.traverse_items(closure);
    println!("{:?}", shopping_cart);

    let mut total_price = 0.0;

    let nested_closure = |item: &mut SupermarketItem| {
        total_price += item.price;
    };

    let closure = |mut shopping_cart: ShoppingCart| {
        println!("{:?}", shopping_cart);
        shopping_cart.traverse_items(nested_closure);
    };

    shopping_cart.checkout(closure);

    println!("USD {:0.2?}", total_price);
}

fn double_the_length<T>(vector: &Vec<T>) -> usize {
    vector.len() * 2
}

fn last_two<T>(slice: &[T]) -> &[T] {
    let len = slice.len();
    let start = {
        if len < 2 {
            0
        } else {
            len - 2
        }
    };
    &slice[start..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    let last = {
        if text.len() < 5 {
            text.len()
        } else {
            5
        }
    };
    &text[..last]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &'a str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        if second.contains(target) {
            second
        } else {
            target
        }
    }
}

fn main() {
    println!("{:?}", double_the_length(&vec![1, 2, 3]));
    println!("{:?}", double_the_length(&vec![1, 2, 3, 4]));

    println!("{:?}", last_two(&vec![1]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4]));

    println!("{:?}", first_five("refrigerator", "Hello"));
    println!("{:?}", first_five("yessssir", "Voice"));

    println!(
        "{:?}",
        find_string_that_has_content("programming", "dining", "gram")
    );
    println!(
        "{:?}",
        find_string_that_has_content("Chili", "Cheese", "ese")
    );
    println!(
        "{:?}",
        find_string_that_has_content("Mountain", "Floor", "hello")
    );
}

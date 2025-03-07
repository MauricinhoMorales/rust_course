fn main() {
    let value: i32 = 1_337;
    let _new_value = value as i16;

    let floating_value = 1.28878;
    println!("{:.3}", floating_value);

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    println!("{is_my_type_of_coffee} and {is_acceptable_coffee}");

    let array: [i8; 4] = [1, 2, 3, 4];
    dbg!(array);

    let person = (1, 1.2, true, array);
    dbg!(person);
}

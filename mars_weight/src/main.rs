use std::io;

fn main() {
    println!("Enter your earthly weight");

    let earth_weight = get_numeric_input();

    let mars_weight = calculate_weight_on_mars(earth_weight);

    println!("Your weight on mars would be {mars_weight}")
}

fn get_numeric_input() -> f32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: f32 = match input.trim().parse() {
        Ok(result) => result,
        Err(_) => panic!("A number is expected"),
    };

    return input;
}

fn calculate_weight_on_mars(earth_weight: f32) -> f32 {
    earth_weight / 9.81 * 3.711
}

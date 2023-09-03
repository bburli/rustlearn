use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your earth weight in kg: ");
    io::stdin().read_line(&mut input).unwrap();
    let earth_weight = input.trim().parse().unwrap();

    println!("Weight on mars is {} kg", calc_weight_on_mars(earth_weight)); // need to reduce weight!
}

fn calc_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your earth weight in kg: ");
    io::stdin().read_line(&mut input).unwrap();
    let earth_weight = input.trim().parse().unwrap();

    println!("Weight on mars is {} kg", calc_weight_on_mars(earth_weight));
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // return can be skipped if it's an expression
}

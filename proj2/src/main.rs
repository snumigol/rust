use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on mars: {}g", mars_weight);
    
}

fn calculate_weight_on_mars(weight: f32) -> f32 {

    (weight/9.81) *3.711
    
}
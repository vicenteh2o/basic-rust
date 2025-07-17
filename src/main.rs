#[allow(unused_variables)]
fn main() {
    open_store("Don Bosco");
    bake_pizza(5, "Cheese");
    swim_in_profit();
    
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number:i32, topping:&str) {
    println!("Baking {number} {topping} pizzas");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}
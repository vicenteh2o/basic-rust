#[allow(unused_variables)]
fn main() {
    open_store("Don Bosco");
    bake_pizza(5, "Cheese");
    swim_in_profit();
    
    let result = square(5);
    println!("{result}");

    // unit: its an empty tuple
    let unit_result = ();
    let unit_result2 = mystery();
    println!("{:?}", unit_result2);

    // blocks
    let multiplier = 3;
    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };
    println!("Calculation: {calculation}");

    let mystery = {
        5 + 4;
    };
    print!("mys: {mystery:?}");
}

fn mystery() {}

fn square(number:i32) -> i32 {
     number * number
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
fn main() {
    let coffe_price = 5.99;
    {
        let coffe_price = 1.99;
        println!("The coffe price is {coffe_price}");
    }
    println!("The coffe price is {coffe_price}")
}

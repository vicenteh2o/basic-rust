const TOUCHDOWN_POINTS: i32 = 6;
#[allow(unused_variables)]
fn main() {
    let season: &str = "Verano";
    let event_time: &str = "06:00";
    let event_time: i32 = 6;
    let mut points_scored: i32 = 28;
    let _favorite_beverage: &str = "Guarana";

    println!("Values: {season} {event_time} {points_scored}");

    points_scored = 35;

    println!("Values: {season} {event_time} {points_scored} {TOUCHDOWN_POINTS}");
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}")
}

#[allow(unused_variables)]
fn main() {
    let age = 17;
    let permited_age = 18;

    if age >= permited_age {
        println!("Its major age.")
    } else {
        println!("Can't enter.")
    }

    let season = "Other"; //"Summer";

    // if season == "Summer" {
    //     println!("school's out!");
    // } else if season == "Winter" {
    //     println!("Vacations!")
    // }

    even_or_odd(7);

    // match (switch)
    let evaluation = true;

    let result = match evaluation {
        true => 1,
        false => 2,
    };
    println!("resul evaluation: {result}");

    match season {
        "Summer" => println!("school's out!"),
        "Winter" => println!("Vacations!"),
        _ => println!("Coffe and rain..."),
    }
    // match evaluation {
    //     true => {
    //         println!("The value is true");
    //     }
    //     false => {
    //         println!("The value is false");
    //     }
    // }

    let number = 8;

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }
}

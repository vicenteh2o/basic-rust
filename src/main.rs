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

    // iterate
    let mut seconds = 10;
    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} sec (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff..");
        seconds -= 1;
    }

    let mut count = 4;
    while count > 0 {
        println!("Count: {count}");
        count -= 1;
    }

    // recursion
    countdown(5);
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{seconds} to blastoff..");
        countdown(seconds - 1);
    }
}

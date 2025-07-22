#[allow(unused_variables)]
fn main() {
    // copy trait
    let time = 2025;
    let year = time;

    // str: stored direct on binary(hard coded in binary executable),
    // not dinamyc
    let food: &str = "pasta";
    // String: store on the heap, mutable
    let text = String::new();
    // string on the heap,
    // stack: reference, length(5) and capacity(10)
    let mut candy = String::from("Kitkat");
    println!("Candy: {candy}");
    candy.push_str(" forever");
    println!("Candy: {candy}");

    // Moves and ownership
    let person = String::from("Vicen");
    // genius new ownership of the heap allocation
    // person become out of scope
    let genius = person;
    // free memory allocation from heap, not stack
    drop(genius);

    // clone: creates 2 copies on heap
    let name_id = String::from("id001");
    let copy_name = name_id.clone();

    println!("Name ID: {name_id}");

    // references and borrowing &
    let my_stack_value = 2;
    let my_int_ref = &my_stack_value;
    println!("Reference value: {my_int_ref}");

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;

    // dereference operator: follows an address
    println!("De-Reference value: {}", *my_int_ref);

    // String(heap), str(hardcode encoded in binary),
    // &String(ref heap), &str(ref to memory loaded from binary)
    let ice_cream = "Cookies and Cream";
    println!("{}", ice_cream);

    // the copy trait with ref
    let ice_cream = "Cookies and Cream"; //address 1
    let dessert = ice_cream; //address 1

    // ownership and fn params
    let oranges = String::from("Oranges");
    print_my_value(oranges); // let value = oranges;
    // oranges not the owner anymore
    // println!("{oranges} is still valid");

    // mutable params
    let burger = String::from("Burger");
    add_fries(burger); // let meal = burger;

    // return values I
    let type_cake = bake_cake();
    println!("I now have a {type_cake} cake");
    // return values II
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    println!("{current_meal}");

    // reference string
    show_my_meal(&current_meal);
}

// meal: String (full ownership with NO permision to modidy)
// mut meal: String (full ownership with permision to modidy)
// meal: &String (ref to string, mem address, with NO permision to update)
// meal: &mut String (ref to string, mem address, with permision to update)
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}")
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate Mousse");
    return cake;
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}

// ownership and fn params
fn print_my_value(value: String) {
    println!("Your value is {value}");
}

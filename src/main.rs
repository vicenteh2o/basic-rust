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

    // String, str and &String, &str
}

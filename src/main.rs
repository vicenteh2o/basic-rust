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
    // alo
}

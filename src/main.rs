mod lib;


use lib::Counter;


fn main() {
    //initialize a counter
    let counter = Counter::new();
    println!("{:#?}", counter);
}
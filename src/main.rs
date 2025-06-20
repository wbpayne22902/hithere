use chrono::Local;
fn main() {
    println!("Hello, world!");
    for i in 1..=5 {
        println!("Number: {}", i);
    }
    println!("Current time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Goodbye, world from Wilhelm!");
}

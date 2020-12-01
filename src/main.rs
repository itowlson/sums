fn main() {
    let mut current = 1;
    let mut total = 0;
    let last = 10; 
    loop {
        println!("Adding square of {}", current);
        total += current * current;
        current = current + 1;
        if current > last {
            println!("Sum of squares: {}", total);
            return;
        }
    }
}

mod core;

fn main() {
    // println!("Enter the numbers separated by comma");

    // let mut line = String::new();

    // let _ = io::stdin().read_line(&mut line);

    // let line = String::from("invalid input");
    let line = String::from("a,a,a,a,a,a,a,1,2,3,a,b,4,5,2,21,2,3,4,5,1,2,3,4,5,6,3,2");
    let mut numbers = core::parse_numbers(line);

    numbers.sort();

    println!("Median {}", core::get_mean(&numbers));

    println!("Mode: {}", core::get_mode(&numbers));
}

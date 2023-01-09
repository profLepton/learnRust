use std::io::stdin;

fn main() {
    println!("Enter a whole number:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");
    println!("The {} number in the pingala series is {}", input, pingala(input));    
}

fn pingala(x: i32) -> i32 {
    if x < 0 {
        pingala(-x);
    }

    if x == 0 || x == 1 {
        return x;
    } 

    else {
        pingala(x-1) + pingala(x-2)
    }

}
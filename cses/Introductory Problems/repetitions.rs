fn main() {

    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).expect("Failed to read from stdin");

    let mut c = 'A';
    let mut ans = 1;
    let mut counter = 0;

    for char in input_string.chars() {
        if c == char {
            counter += 1;
            ans = std::cmp::max(ans, counter);
        }
        else {
            c = char;
            counter = 1;
        }
    }

    println!("{}", ans);
    
}
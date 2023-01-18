fn main() {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Stdin error");

    let n = input_string.trim().parse::<u128>().unwrap();

    let mut x = 5;

    let mut ans = 0;

    while x <= n {
        ans += n/x;
        x *= 5;
    }

    println!("{}", ans);
}
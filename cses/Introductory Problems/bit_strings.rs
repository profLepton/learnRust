
fn main() {
    let mut input_string = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Stdin error");

    let n: u128 = input_string.trim().parse().expect("NAN");
    // 2_u128.pow(n as u32) % (10_u128.pow(9) + 7)
    println!("{}", pow2(n));

}   

fn pow2(mut x: u128) -> u128 {
    let mut res: u128 = 1;
    let mut a: u128 = 2;
    let Mod = 10_u128.pow(9) + 7;
    while x > 0 {
        if (x & 1) != 0 {
            res = (res * a) % Mod;
        }
        a = (a * a) % Mod;
        x = x >> 1;
    }
    res
}
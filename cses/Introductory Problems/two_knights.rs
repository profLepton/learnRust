fn main() {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("stdin failed");

    let n: u128 = input_string.trim().parse().expect("NAN");

    for i in 1..=n {
        let ans = match i {
            1 => 0,
            2 => 6,
            _ => combinations_2(i) - 4 * (i - 2) * ( i - 1), //-( 4 * (n-2) * (n-1)),
        };
        println!("{}", ans);

    }
}


fn combinations_2(n : u128) -> u128{
    (n.pow(2)* (n.pow(2)-1) )/ 2
}
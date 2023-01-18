fn main() {

    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).expect("stdin error");

    let t: u128 = input_string.trim().parse().unwrap();

    for _ in 0..t {

        let mut input_line_string = String::new();
        std::io::stdin().read_line(&mut input_line_string).expect("stdin error");

        let vec: Vec<u128> = input_line_string.trim().split(' ').flat_map(str::parse::<u128>)
                                                    .collect::<Vec<_>>();

        coin_piles(vec[0], vec[1]);

       
    }




}


fn coin_piles(mut x : u128, mut y: u128) {
    if std::cmp::max(x, y) > 2*std::cmp::min(x,y) {
        println!("NO");
        return;
    }
    x %=3;
    y %=3;

    if std::cmp::max(x, y) == 2 && std::cmp::min(x, y) == 1 {
        println!("YES");
    }
    else if x == y && x == 0{
        println!("YES");
    }
    else {
        println!("NO");
    }

}
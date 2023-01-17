fn main() {

    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).expect("Stdin failed");

    let n: u128 = input_string.trim().parse().expect("Nan");

    let mut ans;
    for _ in 0..n {

        let mut input_case_str = String::new();

        std::io::stdin().read_line(&mut input_case_str).expect("Stdin failed");

        let vec: Vec<u128> = input_case_str.split_whitespace().map( |x| x.parse::<u128>().expect("parse eroor")).collect::<Vec<u128>>();

        let  y: u128 = vec[0];
        let  x: u128 = vec[1];

        let  max: u128 = std::cmp::max(x, y);
        let max2: u128 = max.pow(2);

        if max2 % 2 == 0 {
            ans = max2 - (x-1) - (max - y);
        } else {
            ans = max2 - (y-1) - (max - x);
        }

        
        println!("{}", ans);
    }

    

}
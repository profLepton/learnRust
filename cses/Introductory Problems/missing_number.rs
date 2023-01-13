fn main() {

    let mut first_input_string = String::new();

    std::io::stdin().read_line(&mut first_input_string).expect("Error!");

    let n = first_input_string.trim().parse::<u64>().expect("NAN");

    let mut second_input_string = String::new();

    std::io::stdin().read_line(&mut second_input_string).expect("Error!");

    let nums = second_input_string.trim().split(' ').flat_map(str::parse::<u64>)
                                            .collect::<Vec<_>>();

    let input_sum: u64 = nums.iter().sum();

    let calc_sum = n * (n+1) / 2;

    println!("{}", calc_sum - input_sum);

}
fn main() {
    

    let mut _first_input_string = String::new();

    std::io::stdin().read_line(&mut _first_input_string).expect("Error!");

    let mut second_input_string = String::new();

    std::io::stdin().read_line(&mut second_input_string).expect("Error!");

    let nums = second_input_string.trim().split(' ').flat_map(str::parse::<u64>)
                                            .collect::<Vec<_>>();


    let mut ans = 0;
    let mut high = nums[0];

    for num in nums {
        if high > num {
            ans += high - num;
        } else {
            high = num;
        }
    }

    println!("{}", ans);
}
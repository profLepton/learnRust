fn main() {
    let mut input_string = String::new();
    
    std::io::stdin().read_line(&mut input_string).expect("read failed");

    let n: u64 = input_string.trim().parse().expect("NAN");

    match n {
        1 => {println!("{}", 1); return },
        2 | 3 => { println!("NO SOLUTION"); return },
        4 => {
            println!("2 4 1 3"); return;
        }
        _ => (),
    }


        for i in (1..=n).step_by(2) {
            print!("{} ",i);
        }

        for i in (2..=n).step_by(2) {
            print!("{} ",i);
        }
    


}
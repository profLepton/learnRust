use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin!");

    let mut n: u64 = input_text.trim().parse::<u64>().expect("Not a u32!");

    //println!("{}", n);

    while n != 1 {
        print!("{} ", n);

        match n % 2 {
            0 => n = n / 2,
            1 => n = n * 3 + 1,
            _ => unreachable!(),
        }
    }

    print!("1");
}

fn main() {

    let mut input_string: String = String::new();

    std::io::stdin().read_line(&mut input_string).expect("Stdin error");

    let n = input_string.trim().parse::<u128>().unwrap();
    println!("{}", 2_i32.pow(n as u32) - 1);
    move_disks(n, '1', '2', '3');

}

fn move_disks(n : u128, from: char, mid: char, to: char) {
    if n == 0 {
        return;
    }
    println!("{from} {mid}");
    move_disks(n-1, from, to, mid);
    move_disks(n-1, mid, to, from);
}
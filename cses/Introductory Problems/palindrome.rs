fn main() {

    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).expect("stdin error");

    let mut arr: [u32; 26] = [0; 26];

    for c in input_string.chars() {
        arr[c as bytes - b'A'] += 1;
    }

    for e in arr.iter() {
        println!("{}", e);
    }

}
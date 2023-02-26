fn main() {

    let mut input_string: String = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("STDIN failed");

    let n = input_string.trim().parse::<u8>().expect("Input is NAN");

    let mut arr = vec![ "0".to_string() , "1".to_string()];

    let mut i = 2;

    while i < (1 << n) {
        
        for j in (0..i).rev() {
            let value = arr[j].clone();
            arr.push(value);
        }
        for j in 0..i {
            arr[j].push_str("0");
        }
        
        for j in i..2*i {
            arr[j].push_str("1");
        }



        i*= 2; 
    }

    for item in arr.iter() {
        println!("{}", item);
    }


}

fn gray_code(n: u32) -> Vec<u32> {

    if n == 0 {
        vec![0]
    }

    else {
    
    let prev = gray_code(n-1);
    let prefix = 1 << ( n - 1) ;
    let mut result = prev.clone();

    for i in (0..prev.len()).rev() {
        result.push(prefix | prev[i]);
    }

    result
    }
}


fn main() {
    let mut input_string = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .unwrap();

    let n = input_string.trim().parse::<u32>().unwrap();

    for item in gray_code(n).iter() {
        println!("{:b}", item);
    }
}

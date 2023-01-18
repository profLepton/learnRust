fn main() {
    let mut input_string = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Stdin error");

    let n: u128 = input_string.trim().parse().expect("NAN");

    let sum = n * (n + 1) / 2;

    if sum % 2 == 1 {
        println!("NO");
        return;
    }

    println!("YES");

    let mut vec1: Vec<u128> = Vec::new();
    let mut vec2: Vec<u128> = Vec::new();

    if n % 2 == 1 {

        vec1.push(n);
        vec2.push(n-1);
        vec2.push(1);

        for i in (2..((n - 1) / 2)).step_by(2) {
            vec1.push(i);
            vec1.push(n - i);
            vec2.push(i + 1);
            vec2.push(n -i -1);
        }
    } else {
        for i in (1..(n/2)).step_by(2) {
            vec1.push(i);
            vec1.push(n - i + 1);
            vec2.push(i + 1);
            vec2.push(n - i);
        }
    }
    println!("{}", vec1.len());

    for item in vec1.iter() {
        print!("{} ", item);
    }

    println!("\n{}", vec2.len());

    for item in vec2.iter() {
        print!("{} ", item);
    }
}

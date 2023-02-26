fn main() {

    // Takes an input string

    let mut input_string = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("stdin error");

    // Creates an array that counts the number of times each letter occurs in the input string.

    let mut arr: [u32; 26] = [0; 26];

    for letter in input_string.chars() {
        if letter.is_alphabetic() {
            let index = (letter as u8 - 'A' as u8) as usize;
            arr[index] += 1;
        }
    }

    let mut odd_count = 0;

    //Checks the number of letters that occur an odd number of times.
    
    for item in arr.iter() {
        if item % 2 == 1 {
            odd_count += 1;
        }
    }

    if odd_count > 1 {
        println!("NO SOLUTION");
        return;
    }

    //Start outputting the palindrome
    for i in 0..arr.len() {
        if arr[i] %2 == 0 {
            for _ in 0..arr[i]/2 {
                print!("{}", (i as u8 + 'A' as u8) as char);
            }
        }
    } 
        for i in 0..arr.len() {
        if arr[i] % 2 == 1 {
           for _ in 0..arr[i] {
               print!("{}", (i as u8 + 'A' as u8) as char);
        }
        }
        }
    
    for i in (0..arr.len()).rev() {
        if arr[i] %2 == 0 {
            for _ in 0..arr[i]/2 {
                print!("{}", (i as u8 + 'A' as u8) as char);
            }
        }
    }

}

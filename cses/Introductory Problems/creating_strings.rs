fn main() {
    
    let mut input_string = String::new();

    std::io::stdin()
        .read_line(&mut input_string)
        .unwrap();

    let mut arr = [0; 26];
    let mut len = input_string.len();

    let mut count = 0;

    for c in input_string.chars() {
        let index = c as usize - 'a' as usize;
        if arr[index] == 0 {
            count +=1 ;
        }

        arr[ index ] += 1;
    }

   for i in 0..arr.len() {
    let ch = (i as usize + 'a' as usize ) as char;
    

   } 

}




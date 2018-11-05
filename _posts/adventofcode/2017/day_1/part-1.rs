fn main() {
    let input = "1122";
    
    let mut sum = 0;
    let mut index = 0;

    while index < input.len() {
        let d_curr = input.chars().nth(index).unwrap();
        let mut d_next = input.chars().nth(index).unwrap();
        
        if index == (input.len() - 1) {
            d_next = input.chars().nth(0).unwrap();
        } else {
            d_next = input.chars().nth(index + 1).unwrap();
        }

        if d_curr == d_next {
            sum = sum + d_curr.to_digit(10).unwrap();
        }

        index = index + 1;
    }

    println!("sum: {}", sum);
}
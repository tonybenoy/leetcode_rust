fn main() {
    let numbers = vec![555, 901, 4892, 1771, 1];
    let out = find_count_even_numbers(numbers);
    println!("{}",out);
}
fn find_count_even_numbers(nums: Vec<i32>) -> i32 {
    let mut even = 0;
    for num in nums.iter() {
        let mut counter = 0;
        let mut n = *num;
        loop {
            n =n / 10;
            counter += 1;
            if n==0 {
                break;
            }
        }
        if counter%2==0 {
            even+=1;
        }
    }
    even
}

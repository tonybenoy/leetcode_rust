fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut max_cons = 0;
    let mut temp = 0;
    for i in nums.iter() {
        if *i == 1 {
            if temp == 0 {
                temp = 1;
            } else if prev == 1 {
                temp += 1;
            }
            if temp >= max_cons {
                max_cons = temp;
            }
        } else {
            temp = 0;
        }
        prev = *i;
    }
    max_cons
    }

fn main(){
    let nums = vec![1, 0, 1, 1, 0, 1];
    let g = find_max_consecutive_ones(nums);
    println!("{}", g)
}
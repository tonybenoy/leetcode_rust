fn main(){
    let nums = vec![-7,-3,2,3,11];
    let g = sorted_squares(nums);
    println!("{:?}", g)
}
fn sorted_squares( nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for i in 0..nums.len(){
        res.push(nums[i]*nums[i]);
    }
    for i in 0..res.len(){
        for j in i+1..res.len(){
            if res[j]<res[i]{
            res[j] = res[i]+res[j];
            res[i] =  res[j]-res[i];
            res[j] =  res[j]-res[i];
        }
        }

    }

    res
}
fn main() {
    let nums1 = vec![3,1,2,4];
    let p = sort_array_by_parity(nums1);
    println!("{:?}",p);
}
pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let s = nums.len();
    let mut rp=0;
    let mut lp=s-1;
    let mut res = nums.clone();
    for i in 0..s {
        if nums[i] % 2 == 0 {
            res[rp]=nums[i];
            rp += 1;

        }
        else {
            res[lp]=nums[i];
            lp =lp - 1;
        }
    }
    return res;
}

fn main() {
    let mut nums1 = vec![0,0,1,1,1,2,2,3,3,4];
    let p=remove_duplicates(&mut nums1);
    println!("{:?},{}", nums1,p)
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {return 0}
    let mut n = nums[0];
    let mut p = 0;
    for i in 1..nums.len() {
        if nums[i] != n {
            n = nums[i];
            nums[p+1] = n;
            p = p+1;
        }
    }
    (p+1) as i32
    }
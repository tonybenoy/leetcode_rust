fn main() {
    let mut nums1 = vec![4,5];
    let m = 4;
    let p=remove_element(&mut nums1, m);
    println!("{:?},{}", nums1,p)
}
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut l = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            l+=1;
        }
        else {
             for j in (i..nums.len()).rev() {
                if nums[j] != val {
                    nums[i] = nums[i] + nums[j];
                    nums[j] = nums[i] - nums[j];
                    nums[i] = nums[i] - nums[j];
                    l=l+1;
                    break;
                }
            }
    }}
    return l;
}
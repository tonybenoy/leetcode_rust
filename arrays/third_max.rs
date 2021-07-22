fn main() {
    let nums1 = vec![1,2,2,5,3,5];
    let p = third_max(nums1);
    println!("{}",p);
}
pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut m1 = i32::MIN;
    let mut m2 = i32::MIN;
    let mut m3 = i32::MIN;
    let mut n = nums.clone();
    n.sort_unstable();
    n.dedup();
    for i in 0..n.len() {
        if n[i] > m1 {
            m3=m2;
            m2=m1;
            m1 = n[i];
        }
        else if n[i] > m2 {
            m3=m2;
            m2 = n[i];
        }
        else if n[i] > m3 {
            m3 = n[i];
        }
    }
    if n.len() <3 {
        return m1;
    }
    else {
        return m3;
    }
}

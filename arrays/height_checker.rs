fn main() {
    let nums1 = vec![1,1,4,2,1,3];
    let p = height_checker(nums1);
    println!("{}",p);
}
 pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut new_vec = heights.clone();
    new_vec.sort();
    let mut c = 0;
    for i in 0..heights.len() {
        if new_vec[i] != heights[i] {
            c += 1;
        }
    }
    return c
}

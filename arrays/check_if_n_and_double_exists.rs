fn main() {
    let mut nums1 = vec![-2,0,10,-19,4,6,-8];
    let p=check_if_exist( nums1);
    println!("{}",p)
}
pub fn check_if_exist(arr: Vec<i32>) -> bool {
for i in 0..arr.len() {
    if arr[i] % 2 ==0 {
        let y = arr[i] / 2;
        for j in 0..arr.len() {
            if arr[j] == y && i!=j {
                return true;
            }
        }
}}
return false;
}
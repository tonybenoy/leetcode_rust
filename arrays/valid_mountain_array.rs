fn main() {
    let nums1 = vec![0,3,2,1];
    let p=valid_mountain_array( nums1);
    println!("{}",p)
}
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {

    let mut mid_found =false;
    let mut mountain = false;
    let mut uphill = false;
    for i in 0..arr.len()-1 {
        if arr[i] == arr[i+1] ||(!uphill && arr[i] > arr[i+1]) {
            return false;
        }
        if arr[i]<arr[i+1] && uphill == false {
            uphill = true;
        }
        if uphill{
            if ! mid_found{
                if arr[i] > arr[i+1] {
                    mid_found = true;
                    mountain = true;
                }
            }
            else {
                if arr[i] < arr[i+1] {
                    return false;
                }
            }
        }
}
    return mountain;}
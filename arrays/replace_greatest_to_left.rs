fn main() {
    let nums1 = vec![17,18,5,4,6,1];
    let p=replace_elements( nums1);
    println!("{:?}",p)
}
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    if arr.len() <2 {
        return vec![-1];
    }
    let mut largest = find_greatest(&arr);
    for i in 0..arr.len()-1 {
        if arr[i] == largest {
            largest = find_greatest(&arr[i..arr.len()]);
        }
        res.push(largest);
    }
    res.push(-1);
    return res;
}

fn find_greatest(arr: &[i32]) -> i32 {
    let mut l:usize=1;
    for i in 1..arr.len() {
        if arr[i]>arr[l]{
            l=i;
        }
    }
    arr[l]
}
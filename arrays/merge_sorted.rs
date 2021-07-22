fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let m = 3;
    let mut nums2 = vec![2,5,6];
    let n = 3;
    merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1)
}
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx = m-1;
    for i in 0..n{
        let mut flag=false;
        for j in 0..m+n{
            if nums2[i as usize] <= nums1[j  as usize]{
                move_and_append(nums1, m+n,nums2[i as usize], j);
                idx=idx+1;
                flag=true;
                break;
            }
    }
    if flag==false{
        idx=idx+1;
        move_and_append(nums1, m+n,nums2[i as usize], idx);
    }
}

}
fn move_and_append(nums: &mut Vec<i32>, m: i32, val: i32,p:i32) {
    if p<m{
    let t = nums[p as usize];
    nums[p as usize] = val;
    move_and_append(nums,m,t,p+1);
    }
}
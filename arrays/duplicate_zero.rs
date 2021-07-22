fn main() {
    let mut numbers = vec![1,0,2,3,0,4,5,0];
    duplicate_zeros(& mut numbers);
    println!("{:?}",numbers)
}
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i= 0;
    let l = arr.len();
    while i<l {
        if arr[i]==0 {
            let mut t = arr[i+1];
            arr[i+1] =0;
            for j in i+2..l {
                arr[j] = t+arr[j];
                t=arr[j]-t;
                arr[j]=arr[j]-t;
            }
            i=2+i;
        }
        else{
        i=i+1;}
    }
    }
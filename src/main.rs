
fn main() {
    let mut arr = [2, 6, 9, 1, 4];
    sort_array(&mut arr);
    print!("{:?}",arr);
}

fn sort_array(arr:&mut [i32;5]) {
    for i in 0..arr.len()-1 {
        for j  in 0 .. arr.len()-i-1 {
            if (arr[j] > arr[j+1]) {
               let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}
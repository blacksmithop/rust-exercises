fn main() {
    let mut arr: [i32; 5] = [4, 5, 1, 3, 2];

    for i in 1..arr.len(){
        let mut j = i;

        while j>0 && arr[j-1] > arr[j]{
            arr.swap(j-1, j);
            j -= 1;
        }
    }
    print!("{:?}", arr);

}

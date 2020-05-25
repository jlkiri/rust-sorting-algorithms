/* fn bubble_sort(array: &Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                let tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
            }
        }
    }
} */

fn merge(l_half: Vec<i32>, r_half: Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::new();
    let mut mut_l_half = l_half;
    let mut mut_r_half = r_half;

    println!("{} {}", mut_l_half.len(), mut_r_half.len());

    while mut_l_half.len() > 0 && mut_r_half.len() > 0 {
        mut_l_half.split_off(1);
        mut_r_half.split_off(1);
        if mut_l_half[0] < mut_r_half[0] {
            sorted.push(mut_l_half[0]);
        } else {
            sorted.push(mut_r_half[0]);
        }
    }

    sorted
}

fn merge_sort(array: &[i32]) -> Vec<i32> {
    if array.len() < 2 {
        let mut v = Vec::new();
        v.extend_from_slice(array);
        return v;
    }

    let middle = array.len() / 2;
    let left = &array[..middle - 1];
    let right = &array[middle..];

    merge(merge_sort(left), merge_sort(right))
}

fn main() {
    let array = vec![5, 4, 3, 2, 1];
    //bubble_sort(&mut array);
    //println!("{:?}", array);
    let sorted = merge_sort(&array[..]);
    println!("{:?}", sorted)
}

fn bubble_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                // let tmp = array[j];
                // array[j] = array[j + 1];
                // array[j + 1] = tmp;
                array.swap(j, j + 1);
            }
        }
    }
}

fn merge(l_arr: &[i32], r_arr: &[i32], sorted: &mut [i32]) {
    // Current loop position in left half, right half, and sorted vector
    let (mut left, mut right, mut i) = (0, 0, 0);

    while left < l_arr.len() && right < r_arr.len() {
        if l_arr[left] <= r_arr[right] {
            sorted[i] = l_arr[left];
            i += 1;
            left += 1;
        } else {
            sorted[i] = r_arr[right];
            i += 1;
            right += 1;
        }
    }

    if left < l_arr.len() {
        // If there is anything left in the left half append it after sorted members
        sorted[i..].copy_from_slice(&l_arr[left..]);
    }

    if right < r_arr.len() {
        // If there is anything left in the right half append it after sorted members
        sorted[i..].copy_from_slice(&r_arr[right..]);
    }
}

fn merge_sort(array: &mut [i32]) {
    let length = array.len();
    let middle = length / 2;
    if length < 2 {
        return; // No need to sort vectors with one element
    }

    let mut sorted = array.to_vec();

    merge_sort(&mut array[..middle]);
    merge_sort(&mut array[middle..]);

    merge(&array[..middle], &array[middle..], &mut sorted[..]);

    array.copy_from_slice(&sorted); // Copy the sorted result into original vector
}

fn main() {
    let mut array = vec![5, 4, 3, 2, 1];
    merge_sort(&mut array);
    bubble_sort(&mut array);
    println!("{:?}", array);
    //let sorted = merge_sort(&array[..]);
    //println!("{:?}", sorted)
}

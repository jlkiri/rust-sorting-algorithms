
fn partition(array: &mut [i32], l: isize, h: isize) -> isize {
  let pivot = array[h as usize];
  let mut i = l - 1; // Index of the smaller element

  for j in l..h {
    if array[j as usize] <= pivot {
      i = i + 1;
      array.swap(i as usize, j as usize);
    }
  }

  array.swap((i + 1) as usize, h as usize);

  i + 1
}

fn quick_sort_partition(array: &mut [i32], start: isize, end: isize) {
  if start < end && end - start >= 1 {
    let pivot = partition(array, start as isize, end as isize);
    quick_sort_partition(array, start, pivot - 1);
    quick_sort_partition(array, pivot + 1, end);
  }
}

pub fn sort(array: &mut [i32]) {
  let start = 0;
  let end = array.len() - 1;
  quick_sort_partition(array, start, end as isize);
}

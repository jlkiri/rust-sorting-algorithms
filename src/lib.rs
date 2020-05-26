mod bubble;
mod merge;
mod quick;

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn merge_sort_correct() {
    let mut array = vec![5, 4, 3, 2, 1];
    merge::sort(&mut array);
    assert_eq!(array, [1, 2, 3, 4, 5])
  }

  #[test]
  fn bubble_sort_correct() {
    let mut array = vec![5, 4, 3, 2, 1];
    bubble::sort(&mut array);
    assert_eq!(array, [1, 2, 3, 4, 5])
  }

  #[test]
  fn quick_sort_correct() {
    let mut array = vec![5, 4, 3, 2, 1];
    quick::sort(&mut array);
    assert_eq!(array, [1, 2, 3, 4, 5])
  }
}

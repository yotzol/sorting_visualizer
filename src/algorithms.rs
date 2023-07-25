pub fn bubble_sort(arr: &mut [isize], j: usize) {
    if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
    }
}

pub fn selection_sort(arr: &mut [isize], min_idx: usize, j: usize) {
    if arr[j] < arr[min_idx] {
        arr.swap(min_idx, j);
    }
}

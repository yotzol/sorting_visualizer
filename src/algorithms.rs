pub fn bubble_sort(arr: &mut [isize], j: usize) {
    if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
    }
}

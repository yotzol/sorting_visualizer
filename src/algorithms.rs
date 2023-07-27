pub fn bubble_sort(arr: &mut [isize], steps: &mut Vec<Vec<isize>>) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            steps.push(vec![j as isize])
        }
    }
}

pub fn selection_sort(arr: &mut [isize], steps: &mut Vec<Vec<isize>>) {
    for i in 0..arr.len() - 1 {
        for j in i..arr.len() {
            steps.push(vec![i as isize, j as isize]);
        }
    }
}

pub fn insertion_sort(arr: &mut [isize], steps: &mut Vec<Vec<isize>>) {
    let mut arr_copy = arr.to_owned();
    for i in 1..arr_copy.len() {
        let mut j = i;
        while j > 0 && arr_copy[j - 1] > arr_copy[j] {
            arr_copy.swap(j, j - 1);
            steps.push(vec![j as isize]);
            j -= 1;
        }
    }
}

pub enum MergeStep {
    Compare(usize),
    Merge(usize, usize, usize),
}

pub fn merge(arr: &mut [isize], start: usize, mid: usize, end: usize, steps: &mut Vec<MergeStep>) {
    let mut left = arr[start..=mid].to_vec();
    let mut right = arr[mid + 1..=end].to_vec();
    let mut merged = vec![];

    left.push(isize::MAX);
    right.push(isize::MAX);

    let mut i = 0;
    let mut j = 0;

    for _k in start..=end {
        steps.push(MergeStep::Compare(start + i));
        steps.push(MergeStep::Compare(mid + j));
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    steps.push(MergeStep::Merge(start, mid, end));
}

pub fn merge_sort(arr: &mut [isize], start: usize, end: usize, steps: &mut Vec<MergeStep>) {
    if start < end {
        let mid = start + (end - start) / 2;
        merge_sort(arr, start, mid, steps);
        merge_sort(arr, mid + 1, end, steps);
        merge(arr, start, mid, end, steps);
    }
}

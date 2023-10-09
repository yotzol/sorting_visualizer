use std::collections::VecDeque;

pub enum Step {
    Compare(usize, usize),
    Swap(usize, usize),
    Merge(usize, usize, usize),
}

pub fn bubble_sort(arr: &mut [isize], steps: &mut VecDeque<Step>) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            steps.push_back(Step::Compare(j, j + 1));
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                steps.push_back(Step::Swap(j, j + 1));
            }
        }
    }
}

pub fn selection_sort(arr: &mut [isize], steps: &mut VecDeque<Step>) {
    for i in 0..arr.len() - 1 {
        for j in i..arr.len() {
            steps.push_back(Step::Compare(i, j));
            if arr[i] > arr[j] {
                arr.swap(i, j);
                steps.push_back(Step::Swap(i, j));
            }
        }
    }
}

pub fn insertion_sort(arr: &mut [isize], steps: &mut VecDeque<Step>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 {
            steps.push_back(Step::Compare(j - 1, j));
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                steps.push_back(Step::Swap(j - 1, j));
            }
            j -= 1;
        }
    }
}

pub fn merge(arr: &mut [isize], start: usize, mid: usize, end: usize, steps: &mut VecDeque<Step>) {
    let mut left = arr[start..=mid].to_vec();
    let mut right = arr[mid + 1..=end].to_vec();
    let mut merged = vec![];

    left.push(isize::MAX);
    right.push(isize::MAX);

    let mut i = 0;
    let mut j = 0;

    for _k in start..=end {
        steps.push_back(Step::Compare(start + i, mid + j));
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    steps.push_back(Step::Merge(start, mid, end));
}

pub fn merge_sort(arr: &mut [isize], start: usize, end: usize, steps: &mut VecDeque<Step>) {
    if start < end {
        let mid = start + (end - start) / 2;
        merge_sort(arr, start, mid, steps);
        merge_sort(arr, mid + 1, end, steps);
        merge(arr, start, mid, end, steps);
    }
}

pub fn partition(arr: &mut [isize], start: usize, end: usize, steps: &mut VecDeque<Step>) -> usize {
    let pivot = arr[end];
    let mut i = start;

    for j in start..end {
        steps.push_back(Step::Compare(j, end));
        if arr[j] < pivot {
            arr.swap(i, j);
            steps.push_back(Step::Swap(i, j));
            i += 1;
        }
    }

    arr.swap(i, end);
    steps.push_back(Step::Swap(i, end));

    i
}

pub fn quick_sort(arr: &mut [isize], steps: &mut VecDeque<Step>) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((0, arr.len() - 1));

    while let Some((start, end)) = stack.pop() {
        if start < end {
            let pivot = partition(arr, start, end, steps);
            if pivot > 0 {
                stack.push((start, pivot - 1));
            }
            stack.push((pivot + 1, end));
        }
    }
}

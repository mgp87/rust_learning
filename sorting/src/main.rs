fn selection_sort(array: &mut Vec<i8>) -> Vec<i8> {
    // O(n^2)
    for i in 0..array.len() - 1 {
        let mut smallest = i;
        for j in (i+1)..array.len() {
            if array[j] < array[smallest] {
                smallest = j;
            }
        }
        array.swap(smallest, i);
    }
    array.to_vec()
}

fn bubble_sort(array: &mut Vec<i8>) -> Vec<i8> {
    let mut sorted = true;
    for _ in 1..array.len() - 1 {
        sorted = true;
        for j in 0..=array.len() - 2 {
            if array[j] > array[j + 1]{
                array.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    array.to_vec()
}

fn merge_sort(array: &mut [i32]) -> Vec<i32> {
    if array.len() > 1 {
        let mid = array.len() / 2;
        merge_sort(&mut array[..mid]); // left half
        merge_sort(&mut array[mid..]); // right half

        merge(array, mid);
    }
    array.to_vec()
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec(); // left half
    let right = arr[mid..].to_vec(); // right half

    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn quick_sort(arr: &mut [i32], low: usize, high: usize) -> Vec<i32>{
    if low < high {
        let partition_index = partition(arr, low, high);
        let left = quick_sort(arr, low, partition_index - 1);
        let right = quick_sort(arr, partition_index + 1, high);
    }

    arr.to_vec()
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize{
    let pivot = high;
    let mut i = low;
    let mut j = high-1;

    for j in low..=high - 1 {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

fn main() {
    // Selection Sort
    let mut arr: Vec<i8> = vec![4,3,2,1];
    println!("Before selection sorting: {:?}", arr);
    selection_sort(&mut arr);
    println!("After selection sorting: {:?}", arr);

    let mut arr2: Vec<i8> = vec![5,10,1,4,11];
    println!("Before selection sorting: {:?}", arr2);
    selection_sort(&mut arr2);
    println!("After selection sorting: {:?}", arr2);

    // Bubble Sort
    let mut arr3: Vec<i8> = vec![5,4,1,3,2];
    println!("Before bubble sorting: {:?}", arr3);
    bubble_sort(&mut arr3);
    println!("After bubble sorting: {:?}", arr3);

    let mut arr4: Vec<i8> = vec![5,1,4,2,8];
    println!("Before bubble sorting: {:?}", arr4);
    bubble_sort(&mut arr4);
    println!("After bubble sorting: {:?}", arr4);

    // Merge Sort
    let mut arr5: Vec<i32> = vec![4,7,3,5,1,2];
    println!("Before merge sorting: {:?}", arr5);
    merge_sort(&mut arr5);
    println!("After merge sorting: {:?}", arr5);

    // Quick Sort
    let mut arr6: Vec<i32> = vec![8,5,1,2,7,3,4];
    println!("Before quick sorting: {:?}", arr6);
    let len = arr6.len();
    quick_sort(&mut arr6, 0, len - 1);
    println!("After quick sorting: {:?}", arr6);
}

fn fact(num: i32) -> i32 {
    if num > 1 {
        return num * fact(num - 1)
    }else{
        return num
    }
}

fn fibonacci(num: i32) -> i32{
    if num == 0 {
        return 0
    }else if num == 1 {
        return 1
    }
    
    let n1 = fibonacci(num - 1);
    let n2 = fibonacci(num - 2);

    println!("{}", n1 + n2);

    return n1 + n2
}

fn palindrome(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true
    }

    if array[start] == array[end] {
        return palindrome(array, start + 1, end - 1)
    } else {
        return false
    }
}

fn tower_of_hanoi(n: i32) -> i32 {
    if n == 0 {
        return 0
    }

    return tower_of_hanoi(n-1) + 1 + tower_of_hanoi(n-1)
}

// Wrong!
fn sum_of_triangle_array(input: &Vec<i32>) -> Vec<i32> {
    if input.len() == 1 || input.len() == 0 {
        return input.clone()
    }

    println!("{:?}", input);

    let mut tmp_vec = vec![];

    for i in 0..input.len() - 1 {
        tmp_vec.push(input[i] + input[i+1]);
    }

    tmp_vec = sum_of_triangle_array(&tmp_vec);

    tmp_vec.clone()
}

// Good!
fn triangle_solution(arr: &mut Vec<i8>, size: usize) {
    if size < 1 {
        return
    }

    let mut tmp: Vec<i8> = Vec::new();

    for i in 0..size - 1 {
        let x = arr[i] + arr[i+1];
        tmp.push(x);
    }

    triangle_solution(&mut tmp, size - 1);

    println!("{:?}", arr);
}

fn main() {
    println!("{}", fact(5));
    println!("{}", fibonacci(15));

    let array = vec![1,2,3,4];
    println!("{:?}", palindrome(&array, 0, array.len() - 1));

    let array2 = vec![1,2,3,4,3,2,1];
    println!("{:?}", palindrome(&array2, 0, array2.len() - 1));

    println!("{}", tower_of_hanoi(0));
    println!("{}", tower_of_hanoi(1));
    println!("{}", tower_of_hanoi(2));
    println!("{}", tower_of_hanoi(3));
    println!("{}", tower_of_hanoi(4));

    // Using wrong one :(
    let a = vec![1,2,3,4,5];
    println!("{:?}", sum_of_triangle_array(&a));

    // Using good one :)
    let mut vec = vec![1,2,3,4,5];
    let size = vec.len();
    triangle_solution(&mut vec, size);
}

// Finish the solution so that it sorts the passed in array of numbers. If the function passes in an empty array or null/nil value then it should return an empty array.

// For example:

// sort_numbers(&vec![1, 2, 3, 10, 5]); // should return vec![1, 2, 3, 5, 10]
// sort_numbers(&vec![]); // should return !vec[]

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut xs = arr.clone();
    xs.sort();
    xs
}
//----------------------Filtering 

// filter with a predicate,
//a predicate is a function or a closure that returns a bool(tru or false)




// fn is_even(n: &i32) -> bool {
//     n%2 == 0
// }


// fn main() {
//     let numbers = vec![1,3,5,63,7,9,44,66,8];

//     let even_numbers: Vec<i32> = numbers
//     .into_iter()
//     .filter(is_even)
//     .collect();

//     println!("{:?}", even_numbers);
// }


//using closures 



fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,9];

    let even_numbers:Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2==0)
        .collect();

    println!("{:?}", even_numbers)
    
}








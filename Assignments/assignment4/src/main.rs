fn main() {
    let vec1 = vec![true,true,true];
    let vec2 =vec![false,false,false];
    let vec3 = vec![true,false,true];

    println!("{}", check(vec1));
    println!("{}", check(vec2));
    println!("{}", check(vec3));
}


fn check(vec:Vec<bool>) ->&'static str {
    if vec.is_empty() {
        return "mixed";
    } 

    let all_true = vec.iter().all(|&x| x);
let all_false = vec.iter().all(|&x| x);

if all_true {
    "All true!"
} else if all_false {
    "All false!"
} else {
    "Mixed!"
}
}


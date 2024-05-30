// Varibles in rust 

//using let keyword

fn main(){

    using_let();
    using_const();
    known_string();
    unknown_string();
    using_if_else();
    fn_loop();
}

fn using_let(){
    let a = 10; // a is immutable by default hence cannot be assigned another value

    let  b = 20; // b is mutable and can be assigned another value

    let  c = b; // assigning b a new value

    println!("values for a, b and c {}, {}, {} ", a,b,c);
}

fn using_const(){
    const X:i32 = 50; // constansts cannot be changed by default 
    println!("Value of x is {}", X);// when dealing with const variables have to be capital letters
    //constants cannot be shadowed

}

// in rust we have a type for each variables

//for integers we have signed and unsigned denote by (usize and isize)
//floats are denoted by (f32, f64)


//    string literals

// we have &str and string

//&str is used when the value of the string is known at compile time 


fn known_string (){
    let company:&str = "Web3Clubs";
    let user:&str = "Hezron";

    println!("Company is {} and the user is {}", company, user);
}


//String is used when the string is unknown at compile time 


fn unknown_string(){
    println!("Please enter your name:");
    
    let user = String::from("Hezron");
    println!("Please enter your age:");
    let age = String::from("21");

    println!("The user {} and the age is {}", user, age);
}


// decision making 

//using if else 

fn using_if_else(){
    let nums = 6;

    println!("print the nums {:?}", nums);

    if nums % 2==0 {
        println!("number is even");
    } else {
        println!("number is odd");
    }
}

//using loops

fn fn_loop(){
    let mut y = 0;

    while y < 10 {
        y+=1;
        println!("Inside loop y is {}", y);
    }
    println!("outside loop y value is {}", y);
}
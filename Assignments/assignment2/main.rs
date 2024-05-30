///Converting temperature using Match statements and enums

use std::io;


enum Tempereature {
    Celsius,
    Fahrenheit
}


fn main() {
  println!("Welcome to Temp converter \n Please Enter temperature");

  let mut temp = String::new();
  

  io::stdin().read_line(&mut temp).expect("unable to get the temp");

  let temp: f64 = temp.trim().parse().expect("Error");

  println!("Select F to convert to fahrenheit and C to convert to Celsius");

  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("Invalid option");

  let input = input.trim().to_lowercase();

  let converter = match input.as_str() {
    "f" => Tempereature::Fahrenheit,
    "c" => Tempereature::Celsius,

    _=> {
        println!("Invalid option");
        return;
    }
  };

  let result = match converter {
      Tempereature::Celsius => {
        temp*32.0
      },
      Tempereature::Fahrenheit => {
        temp / 32.0
      }
  };

  println!("The temp is {}", result);
}
fn main() {
    //let's count how many time
    let mut count = 0;
    //this will run forever unless we give it an ending condition
    //it's also an expression, so whatever we break with, we return
    let result = loop {
        println!("This code has run {} time(s)", count + 1);

        //here's the ending condition
        if count == 4 {
            break "Chicken";
        }
        //count starts at 0, we need to increase it untill it reaches the ending condition
        count += 1;
    }; //need to end a loop with a semicolon.
    println!("{}",result);
    
    //let's declare a new mutable variable.
    let mut number = 5;

    //here's a "while" loop that runs while number less than or equal to 8
    while number <= 8 {
        println!("{}",number);

        number += 1;
    }
    println!("Soundoff!");

    //define an array with name arr
    let arr = [10,9,8,7];

    //a for loop that is incredibly similar to python syntax
    for element in arr {
        println!("This element's value is {}", element);
    }
}
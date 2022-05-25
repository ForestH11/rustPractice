fn main() {
    /*Now it's time for expressions! A simple way to think of an expression
    is something that returns a value. A statement is the opposite of an 
    expression, and it does not return a value*/

    //these are statements
    let x = 5;
    let mut phrase = "taco";

    println!("x is {} and phrase is {}, now let's do something",x,phrase);
    
    //this is an expression
    if x + 4 > 8 {
        phrase = "Eat Bannana";
    } else {
        phrase = "Behold this scene--";
    }
    //and this is a statement
    println!("Your phrase is: {}", phrase);
}
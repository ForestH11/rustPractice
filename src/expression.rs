fn main() {
    /*Now it's time for expressions! A simple way to think of an expression
    is something that returns a value. A statement is the opposite of an 
    expression, and it does not return a value*/

    //these are statements
    let x = 5;
    let mut phrase = "taco";

    println!("x is {} and phrase is {}, now let's do something",x,phrase);
    
    //this is an expression
    phrase = if x + 4 > 8 {
        //we could assign phrase = each of these strings, but since if
        //statements are expressions, we can just set phrase = to the 
        //if statement.

        "Eat Bannana"
    } else {
        "Behold this scene--"
    };
    
    //and this is a statement
    println!("Your phrase is: {}", phrase);
}
fn main() {
    println!("Hello, World!");
    //create a complex type, String that is stored on the
    //heap instead of the stack
    let s = String::from("Hello");
    // - move occurs because `s` has type `String`, 
    //which does not implement the `Copy` trait


    let x = "How are you?";
    another_function();
    
    // - value of `s` moved here
    take_ownership(s);
    
    take_ownership(x.to_string());
    /* The following line of code does not work
    because s is no longer in scope. The function
    take_ownership has s now, and since it is a
    String type, there is no copy made.*/
    // println!("{}",s);
    
    println!("\"{}\" after take_ownership() ",x);
}

fn another_function() {
    println!("Another Function")
}

fn take_ownership(s: String) {
    println!("{}",s);
}
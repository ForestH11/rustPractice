fn main() {
    println!("Hello, World!");
    let s = String::from("Hello");
    let x = 5;
    another_function();
    takeOwnership(s);
    takeOwnership(x);
    // The following line of code does not work
    //because s is no longer in scope. The function
    //takeOwnership has s now, and since it is a
    //String type, there is no copy made.
    // println!("{}",s);
    println!("{}",x);
}

fn another_function() {
    println!("Another Function")
}

fn takeOwnership(s: String) {
    println!("{}",s);
    s;
}
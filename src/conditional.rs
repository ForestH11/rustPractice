fn main() {
    //define some variables (remember they default to immutable)
    let x = 5;
    let y = 11;
    let z = 4;

    //here's a few simple if, else if statements.
    if x > y {
        println!("{} is greater than {}.",x,y);
    } else if x < y {
        println!("{} is less than {}.",x,y);
    }
    if y > z {
        println!("{} is greater than {}.",y,z);
    } else if z < y {
        println!("{} is less than {}.",y,z);
    }
    if x > z {
        println!("{} is greater than {}.",x,z);
    } else if z < x {
        println!("{} is less than {}.",x,z);
    }

    //here's a if statement that returns a value, showing that if statements are expressions
    let w = if x > y {x + 1} else {y +1};

    //some information on how it worked
    println!("The variable w has a value of {}.",w);
    println!("It was determined by checking if {} was greater than {}.",x,y);
    println!("If {} was greater, the value would be {} + 1",x,x);
    println!("Other wise the value would be {} + 1",y);
}
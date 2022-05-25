fn main() {
    //There are two categories of types in Rust. Scalar, and compound.
    //scalar types
    
    //Rusts default int type is i32, which stands for "signed 32 bit"
    let a = 10; 

    // we've explicitly typed this as a signed 8 bit number. 127 is the largest value this type can store
    // and -128 is the lowest.
    let b: i8 = 127; 
    
    // this is unsigned, meaning it can't be negative. But unsigned numbers can make use of one more bit
    // so the highest value for this type is 255, but the lowest is 0.
    let c: u8 = 255;  

    //This is a float, and it defaults to 64 bit, or f64
    let d = 10.0;

    //less storage space, less precision, but a 32 bit float is available if necessary
    let e: f32 = 5.0; 

    //boolean type
    let f = false;

    //explicitly boolean type
    let g: bool = true;
    
    //single character, or char
    let h = 'z';

    //explicitly typed char
    let i:char = 'x';

    /*It's less important to explicitly type your boolean values and characters
    because there's no variation. A char is a char, and a bool is a bool. It matters
    more with Integers and floats, because you can specify how much memory you want 
    set aside for your numbers.*/

    //Compound types

    //Tuple
    
    //A tuple will infer the types inside the tuple
    let j = (10, 'g', 5.0);

    //but you can explicitly declare them
    let k: (i32, char, f64) = (11, 'w', 6.0);

    //to get values out of the tuple, set a tuple of variables equal to the tuple
    let (l, m, n) = k;

    //or we can use dot notation
    let eleven = k.0;
    let double_u = k.1;
    let six_point_zero = k.2;

    //Arrays

    //arrays are collections of the same type of items. They remain a fixed length
    let o = [5, 6, 7, 8, 9];

    //when we define an array, we define the type and the length
    let p: [i32; 5] = [10, 11, 12, 13, 14]; //array with item type i32, length of 5 items.

    //you can create an array of the same item by using the following syntax
    let q: [2; 5]; //this is the same as saying let q = [2, 2, 2, 2, 2];

    //like many other languages, the array is accesed with bracket notation
    let r = p[2] //the item in index 2 would be 12, so r = 12



}
fn main() {
    // ================================
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!");
    // s2 is a shallow copy of s1 + s1 falls out of scope after being copied (avoids double frees)
    // ================================
    
    // ================================
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
    // after reassignment of s, original value "hello" is dropped from the heap
    // ================================

    // ================================
    // use clone to make a deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");
    // ================================

    // ================================
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // dont have to call clone here despite x remaining valid/not being moved to y because types
    // like ints have a known size at compile time and are stored on the stack
    // => copies of actual values are quick
    // no reason to prevent x from being valid after y's creation
    // see: 'Copy' trait
    // ================================
}

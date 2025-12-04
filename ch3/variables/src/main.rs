fn main() {
    let x = 5;

    let x = x + 1;

    {   
        // below is shadowing above
        // allows for transformations without var being mutable + has var being immutable after
        // said changes
        //
        // note: it is considered to be a new variable (so my intuition was correct)

        let x = x * 2;
        println!("Val inner scope: {x}");
    }

    println!("The value of x is: {x}");
    
    // example
    let spaces = "   "; // cba to allow for user input
    let spaces = spaces.len();

    println!("Num spaces: {spaces}");

}

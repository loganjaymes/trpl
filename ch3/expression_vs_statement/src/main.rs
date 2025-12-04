fn main() {
    // stmt; doesnt return, just executes;
    let name = "statement";
    println!("meow {name}");

    // cannot use statement in an expression, ie.
    // let var = (let var1 = 2);
    // unlike in C
    
    // however
    // let var = 10 + 20;
    // 10 + 20 is an expression & \thf returns a value, however the line above remains a statement
    
    // expression (inside of dbg);
    dbg!(20 + 50);
    // since it is an expression we can do
    let res = dbg!(20 + 50);
    
    // ex. 3, below is statement
    let sum = {
        let x = 10;
        let y = 20;
        
        // however, below is expression
        x + y
    };

    dbg1(sum); // debug is used as statement
}

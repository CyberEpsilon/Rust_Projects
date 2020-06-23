fn main() {
    const MAX_POINTS: u32 = 100_000; //constant (cannot be made into mutable values) 

    /*Constants:
     *-declared in any scope
     -set only to constant expression
     -cannot be the result of a function call or other values that are computed at runtime
     -type of a const value must be annotated ( : u32 is the type annotation indicating that u32 is an integer)
     -valid for entire programmer
     */


    //let x = 5; //Immutabilty:currently the variable x is immutable (by default in Rust)
    let mut x = 5; //x is now mutable
    println!("The value of x is: {}", x);
    x = 6;//This statement returns an error because the original x wasn't declared as a mutable variable. Thus, shadowing cannot occur. 
    println!("The value of x is: {}", x);


    //Shadowing: This allows us to change the value of an immutable variable by using the let
    //keyword ; in effect a new variable is created 
    let y = 5;// y first binds to a value of 5
    let y = y + 1;//this new y shadows the first y
    let y = y * 2;//this new y also shadows the second y
    println!("The value y is: {}", y);

    /*DATA TYPES
     * 1.Every value in Rust is of a certain data type 
     * 2. Two data type subsets exist:scalar and compound 
       3. Rust is statically typed (all var types must be known at compile time)
       4. Type inference is based on var's value and how it's used

    */
    let mut spaces = "  ";
    //spaces = spaces.len();//This line generates an error because the expected type for spaces is &str


    //Statements vs. Expressions
    
    //Statements do not return values. You can't assign a statment to antoher variable 
    let y = 6; 

    //

}

fn main() {
    /*FUNCTIONS:
     * -created using fn keyword 
     * Functions can be defined anywhere in the program
     *
     * Format:
     *
     * function signature{
     *  function body
     * }
     */

    //Standard function example
    fn myFunction(){ 
        println!("Hey this function is really cool!");
        let m : u32 = 234; 
        println!("The value of m is: {}", m);
    }
    //Function with parameters
    fn paramFunction(b: i32, a: u32){//This is the FUNCTION SIGNATURE; All parameters MUST be declared with a type
        let sum = b + a;
        println!("The sum of a and b is: {}", sum);
    }

    //Function call with concrete values (arguments
    paramFunction(45,22);  
    

}


fn main() {
    /*This file provides examples of Scalar and Compound data types that are built into Rust.
     * */
    //SCALAR SECTION
    
    //Integer Types (Signed uses "i" and Unsigned uses "u")--8,16,32,64,128bit varieties
    //available,arch
    let myInt = 3; //type inference used; no explicit type annotation; i32 is default integer type
    let myNum : u32 = 45;

    //Floating-Point Types (f32 and f64)
    let x = 2.0; //type infered as float; f64 by default--any other type of float requires type annotation 
    let y : f32 = 3.0; //f32 specified with type annotation 

    //Numeric Operations for Scalar Types

    //addtion
    let sum = 4 + 2;

    //subtraction
    let difference = 65.8 - 42.1;

    //multiplication
    let product = 9 * 23;

    //division
    let quotient = 45.1 / 34.2;
    
    //remainder
    let remainder = 56 % 3; //Modulus operation

    //Boolean Type (true or false values) --1byte
    let t = true; //without explicit type annotation ; type inference 
    let f : bool = false; //with explicit type annotation

    //Character Type (4bytes)-Unicode Scalar Value 
    let c = 'k';
    let myChar = '&';

    //COMPOUND SECTION --Compound types group multiple values into one type; 
    
    /*Tuples: 
     * Fixed Length
     * Comma separated list of values in parentheses ()
     * Each of these values can be of a different type.
     */

    let myTuple : (i32, f64, u8) = (300, 4.5, 2);

    //Printing values of tuple
    let myTuple = (300, 4.5, 2);
    let (x,y,z) = myTuple; //Destructuring: This line turns myTuple into 3 separate values 
    println!("The value of y is: {}", y); //Test to see individual values

    //Direct access of tuple elements 
    let newTuple : (i32, f64, u8) = (200, 3.5, 1); 
    let firstElement = newTuple.0; //index numbering applies 
    let secondElement = newTuple.1;
    let thirdElement = newTuple.2;

    /*Arrays:
     * Fixed Length
     * Values must be of SAME type
     *Comma separated list of values in brackets []
     Useful for when data is allocated on stack rather than the heap 
     Arrays are a single chunk of memory allocated on the stack 
     Useful for insuring fixed number of elements
     NOTE: There are also vectors available--(similar to arraylists that grow and shrink in size)
     */

    //Basic array
    let myArray = [1,2,3,4,5];
    let strArray = ["hey", "how", "are", "you"];

   //Specifying array type using type annotation + specifying number of elements
   let typedArr : [i32; 5] = [1,2,3,4,5];

   //Initializing array with a set of duplicate elements
   let a = [3; 5]//where 3 is the duplicated element and 5 indicates that it's duplicated 5 times; a = [3,3,3,3,3]; 

    //Accessing array elements using indexing
    let charArray = ['e','u','f'];
    let firstChar = charArray[0];
    let secondChar = charArray[1];
    //NOTE: a panic will result if an attempt to access and out-of-bounds index is made BUT invalid
    //memory will not be accessed 

   
}

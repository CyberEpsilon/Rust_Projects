/*Note on PRELUDES: The prelude is a list of programs Rust auto imports; other preludes must be
 * imported manually from their respective libraries. Rust inserts extern crate std; into the
 * crate root of every crate and use std::prelude::v1::*; into every module
 *
 * Crate: a collection of Rust source code files
 * :: format examples (non-exhaustive list): 
 * crate (no ::)
 * crate::module
 * crate::trait
 * crate::module::item(typically a struct, enum, trait or function)
 */
use rand::Rng;//imports crate rand and its associated trait Rng; interpreted together as the trait Rng whereas rand:: 
use std::cmp::Ordering;//imports cmp module from std crate and its enum trait, Ordering; interpreted together as the enum Ordering with variants Less,Greater and Equal. 
use std::io;//imports the standard library with io traits


fn main() {
    println!("Guess the number!");//macro(indicated by !) that prints string
    let secret_number = rand::thread_rng().gen_range(1,101);
    /*thread_rng() is a function asssociated with the rand crate; it's a random number generator
     * that is local to the current thread of execution and seeded by the operating system. 
     * gen_range() is a method called by the thread_rng() function--it is a method defined by the
     * Rng trait and brought into scope by rand::Rng statement 
     * gen_range() generates random number between a lower bound (1st argument) and an upper
     * bound(2nd argument) -1. The range in this case is 1 -> 100. 
     *
     *
     */
    loop {//
        println!("Please input your guess.");

        let mut guess = String::new();//mutable variable guess is bound to value String::new() --constructor that returns instance of String 
        //println!("The secret number is: {}", secret_number);
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        /*This line does several things: 
         * 1. stdin() funcion returns instance of std::io::Stdin; a style that represents a handle
         *    to standard input 
         * 2. .read_line() calls read_line method on standard input handle to get input from the
         *    user; takes user's input into standard input and places that on a string--read_line
         *    takes a string argument and returns an instance of io::Result ---Result is an enumeration type associated with io (and can be a generic or other iteration in a different
         *    submodule). The variants of Result are Ok(contains successfully generated value),
         *    Err(contains info on how/why operation failed)
         * 3. & indicates a reference -- &mut guess makes the the reference mutable; &guess would
         *    be a reference but would be inherently immutable 
         *  4. .expect() is a method that can be called by an instance of io::Result 
         *      Conditions: (1) If io::Result instance produces Err variant, expect() crashes
         *      program and returns successfully generated value that Ok contains (3) If io::Result instance produces Ok variant,
         *      expect() will display the string message argument "Failed to read line."
         */




        
        //Without the line below, a mismatched type error(Rust type inference: can't compare number type with String) will occur
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,//if Result instance returns Ok, return num (guess user entered)
            Err(_) => continue,//if Result instance returns Err, continue
        };
        /*Things to note:
         *1. A variable named guess is created and shadows the previous mutable variable guess
          2. guess is bound to the expression match guess.trim().parse() where guess is the original string that now calls the trim() method that eliminates whitespace at the beginning and the          end. 
          3.the trim() method is also needed because read_line() adds a newline (\n) character at the  end of the string that the user enters ; trim() eliminates the newline character making it 
          suitable for u32
          4. parse() parses (or converts) the string slice returned from trim() and returns message provided if Err occurs or the value if Ok occurs
         *5. Error-Handling:This match expression is being used for error handling. expect() would simply crash the program. If parse() converts successfully, it will return value stored in Ok                 variant; If not, it returns value stored in Err variant; (_) underscore is a catchall value 
          6. let guess: u32 specifies u32 as the number type parse() returns; the (:) indicates 
          variable type annotation
        */

        println!("You guessed: {}", guess);//{} placeholder indicates that value passed to guess var will be concatenated after "You guessed: "

        
        match guess.cmp(&secret_number){ 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;//this allows the program to exit the loop when the user guesses the secret number correctly; This is also the program's exit.
            }
        /*1. The cmp method compares two values; it accepts a reference to the value that is
         *    compared; guess.cmp(&secret_number) compares the secret number to guess
         *2. match (testing condition){
                arm (pattern + code that runs if testing condtion matches pattern)
                arm
                arm...
                NOTE: A native match expression evaluates arm from top-to-bottom exiting after
                a matching arm is found
            }
         *
         *
         */

        }
    }
}
//Notes for improvement: This game should limit the number of incorrect guesses and exit. Alternatively, the user could be given an option to quit. 

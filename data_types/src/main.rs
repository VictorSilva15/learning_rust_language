fn main() {
    // In Some cases, we need give the type to the variable,
    // Thus, it's possible convert the type of string to number for example
    // as show the line bellow. Without the type it's impossible use parse()

    let x: i32 = "-42".parse().expect("Not a Number");

    println!("{}", x);


    /* 
    
    Rust is separated in Scalar and Compound types
    
    
    Scalar: represents a single value. Rust has four
    primary scalar types:
        - Integers
        - Floating-point numbers
        - Booleans
        - Characters
    
    INTEGER TYPE =>
    
    * An Integer without a fractional component. 

    8-bit	  i8	   u8
    16-bit	  i16	   u16
    32-bit	  i32	   u32
    64-bit	  i64	   u64
    128-bit	  i128     u128
    arch	  isize    usize

    In laymen's terms an unsigned int is an integer that can not
    be negative and thus has a higher range of positive values that
    it can assume. A signed int is an integer that can be negative
    but has a lower positive range in exchange for more negative 
    values it can assume.

    signed integer start with "i"
    unsigned integer start with "u"

    unsigned int x -> Unsigned variables can store the number zero and positive numbers.
    signed int y -> Signed variables can store negative, zero, and positive numbers.

    The later numerator is the length that this type can catch up

    Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive,
    where n is the number of bits that variant uses. So an i8 can store numbers 
    from -(2^7) to 2^7 - 1, which equals -128 to 127. Unsigned variants can store 
    numbers from 0 to 2^n - 1, so a u8 can store numbers from 0 to 2^8 - 1, 
    which equals 0 to 255.

    You can write integer literals in any of the forms shown in Table 3-2. Note that number 
    literals that can be multiple numeric types allow a type suffix, such as 57u8, to 
    designate the type. Number literals can also use _ as a visual separator to make the 
    number easier to read, such as 1_000, which will have the same value as if you had 
    specified 1000.

        Number literals	        Example

        Decimal	                98_222
        Hex	                    0xff
        Octal	                0o77
        Binary	                0b1111_0000
        Byte (u8 only)	        b'A'


    Let’s say you have a variable of type u8 that can hold values between 0 and 255. 
    If you try to change the variable to a value outside of that range, such as 256, 
    integer overflow will occur. Rust has some interesting rules involving this behavior. 
    When you’re compiling in debug mode, Rust includes checks for integer overflow that 
    cause your program to panic at runtime if this behavior occurs. Rust uses the term 
    panicking when a program exits with an error;


    To explicitly handle the possibility of overflow, you can use these families of 
    methods that the standard library provides on primitive numeric types:

    - Wrap in all modes with the wrapping_* methods, such as wrapping_add
    - Return the None value if there is overflow with the checked_* methods
    - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    - Saturate at the value’s minimum or maximum values with saturating_* methods

    */


    // FLOATING POINT TYPE =>


    /*
    
    Rust also has two primitive types for floating-point numbers, which are numbers 
    with decimal points. Rust’s floating-point types are f32 and f64, which are 32 
    bits and 64 bits in size, respectively. The default type is f64 because on modern 
    CPUs it’s roughly the same speed as f32 but is capable of more precision.

    Here’s an example that shows floating-point numbers in action:

    */

    let y = 10.6; // f64

    let z: f32 = 3.0; //f32

    println!("The y is: {} and its type is f64", y);
    println!("The z is: {} and its type is f32", z);


    // Basic Operators => 

    /* 
    
    Rust supports the basic mathematical operations you’d expect for all of the number 
    types: addition, subtraction, multiplication, division, and remainder. Integer 
    division rounds down to the nearest integer. The following code shows how you’d 
    use each numeric operation in a let statement:
    
    */
    
    //Addition
    let sum = 5 + 10;

    //Subtraction
    let difference = 95.5 - 4.3;
    //Multiplication
    let product = 4 * 30;
    //Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; 
    //Remainder
    let remainder = 43 % 5;


    println!("The sum of 5 + 10 = {}", sum);
    println!("The subtraction of 95.5 - 4.3 = {}", difference);
    println!("The product of 4 * 30 = {}", product);
    println!("The quotient of 56.7 / 32.3 = {}", quotient);
    println!("The floored of 2 / 3 = {}", floored);
    println!("The remainder of 43 % 5 = {}", remainder);

    // BOOLEAN =>   

    /*
    As in most other programming languages, a Boolean type in Rust has two 
    possible values: true and false. Booleans are one byte in size. 
    The Boolean type in Rust is specified using bool. For example:

    */


    let t = true;

    let f: bool = false;


    println!("t is {}", t);
    println!("f is {}", f);


    // CHARACTER TYPE =>


    /*

    So far we’ve worked only with numbers, but Rust supports letters too. 
    Rust’s char type is the language’s most primitive alphabetic type, and 
    the following code shows one way to use it. (Note that char literals 
    are specified with single quotes, as opposed to string literals, 
    which use double quotes).
    
    */
    
    let c = 'z';
    let d = 'ℤ';

    let heart_eyed_cat = '😻';


    print!("\n --- CHARACTER TYPE --- \n\n");
    println!(" {}, {}, {}", c, d, heart_eyed_cat);



    // COMPOUND TYPES =>

    // Rust has two primitive compound types: Tuples and Arrays:

    // Tuples:

    /*

    A tuple is a general way of grouping together a number of values with a variety 
    of types into one compound type. Tuples have a fixed length: once declared, 
    they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses. 
    Each position in the tuple has a type, and the types of the different values in 
    the tuple don’t have to be the same. We’ve added optional type annotations in 
    this example:

    */

    let mut tup = (500, 6.4,'A');
    
    println!("\n --- COMPOUND TYPES --- \n");
    println!("{}", tup.2);

    tup.2 = 'B';

    println!("{}", tup.2);
    //Another way to catch the value stored in the tuple:

    let (n1, n2, char1) = tup;

    println!(" {}, {}, {}", n1, n2, char1);



    // Array:


    /*

    Another way to have a collection of multiple values is with an array. 
    Unlike a tuple, every element of an array must have the same type. 
    Arrays in Rust are different from arrays in some other languages 
    because arrays in Rust have a fixed length, like tuples.

    In Rust, the values going into an array are written as a comma-separated 
    list inside square brackets:
    
    */


    let my_array = [1,2,3,4,5];

    println!("\n ARRAYS \n");
    println!("{}", my_array[0]);


    //Another way to create an array:
    let my_other_array: [char; 3] = ['J','K','L'];

    println!("{}", my_other_array[1]);

    //Another way to create an array:
    let another_array = [3;5];

    /*
    
    The array named another_array will contain 5 elements that will all be set to the 
    value 3 initially. This is the same as writing let another_array = [3, 3, 3, 3, 3]; 
    but in a more concise way.

    */

    println!("{}",another_array[2]);

    //Bitwise Operators = 
    
    let bw1 = 1;
    let bw2 = 2;

    let bwres1 = bw1 & bw2; // 0 (01 && 10 -> 00)
    let bwres2 = bw1 | bw2; // 3 (01 || 10 -> 11)
    let bwres3 = bw1 ^ bw2; // 3 (01 != 10 -> 11)
    let bwres4 = bw1 << bw2; // 4 (add b number of 0s to the end of a ->
    //'01' + '00' -> '100')
    let bwres5 = bw1 >> bw2; // 0 (Remove b number of bits from the end of a -> 0-1- -> 0)
    
    println!("\n\n --- Using Bitwise --- \n\n{}\n{}\n{}\n{}\n{}",
        bwres1,bwres2,bwres3,bwres4,bwres5);
}

fn main() {
    let mut x = 5; // let x = 5; immutable and trhows error on next line
    println!("The values of x is: {x}");

    x = 6;
    println!("The values of x is: {x}");
    
    // Shadowing
    let x = 7;
    println!("The values of x is: {x}");

    let x = x+1;
    
    {
        let x = x*2;
        println!("The values of x in the inner scope is: {x}");
    }
    
    println!("The values of x is: {x}");

    // unmutable shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The values of spaces is: {spaces}");

    // mutable shadowing - invalid
    // let mut spaces = "   ";
    // spaces = spaces.len(); // error: cannot change variable type
    let number :u32 = 1_000_000; // 1_000_000 is the same as 1000000: just visually easier to read
    println!("The values of number is: {number}");
    
    my_first_function();

    exploring_tuples();
    exploring_arrays();
    arguemental_func(100);
    println!("Gimme five! ... *{}*", return_a_value_func());
    println!("100 + 1 is: {}", additional_func(100));
    conditional_expressions();
    concoction_of_if();
    loopy_loop_and_pull();
}

fn my_first_function() {
    println!("Technically, this is my second function because of main()");
}

fn exploring_tuples(){
    println!("Tuples!");
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    println!("The values of tup is: {tup:?}");
    println!("The value of the first element of tup is: {}", tup.0); // dot notation has to be used outside of {}
    println!("The value of the second element of tup is: {}", tup.1);
    println!("The value of the third element of tup is: {}", tup.2);
    let (x,y,z) = tup; // destructuring
    println!("The values of x,y,z is: {x},{y},{z}");
}

fn exploring_arrays(){
    println!("Arrays!");
    let arr = [1, 2, 3, 4, 5];
    println!("The values of arr is: {arr:?}");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The values of a is: {a:?}");
    let a_b = [3; 5];
    println!("The values of a_b is: {a_b:?}");
}

fn arguemental_func(arguement: u32) {
    println!("The arguement is: {}", arguement);
}

fn return_a_value_func() -> u32 {
    5 // Wild that this is valid... i hate it
}

fn additional_func(arguement: u32) -> u32 {
    arguement + 1
}

fn conditional_expressions() {
    let number = 3;
    if number < 5{ // condition must result in a boolean. does not automatically convert to bool!
        println!("Condition is true!");
    } else if number == 5{
        println!("We got 5 B) !");
    } else {
        println!("Condition is false!");
    }
}

fn concoction_of_if() {
    let condition :bool = true;
    let number :u8 = if condition { 1 } else { 2 }; // values must be of the same type
    println!("The value of number is: {number}");
}

fn loopy_loop_and_pull() {
    // loop, while, for
    // loop is basically 'while true'
    let mut counter : u32 = 0;
    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2; // this will return counter * 2
            // break exists loop, return exits function
        }
    };

    println!("result = {result}");

    counter = 0;
    // loop with a label
    'count_up: loop { // helpful when nesting loops
        counter += 1;

        if counter == 10{
            break 'count_up; // need to specify what loop to break if nested
        }
    }

    for number in (1..4){
        println!("{number}!");
    }
    println!("Hey look at that... we can count ;)! nice");

    println!("now your shoes all lookin cool");
}
fn main() {
    

    let s1 = String::from("hello");
    // let s2 = s1; // error: s1 is dropped from memory here, now belongs to s2
    let s2 = s1.clone(); // s2 is now a copy of s1: but expensive for resources

    println!("s1 = {s1}, s2 = {s2}");

    // Crazy but this one is valid!
    let x = 5;
    let y = x; // valid because this is on the stack and copying is inexpensive

    println!("x = {x}, y = {y}");

    // Some surprising things can happen with moves
    take_ownership(s1);
    // println!("s1 is now: {s1}"); s1 is no longer valid because it was moved!
    // s2 = take_and_return_ownership(s2); // doesnt work because s2 isnt mutable
    let s3 = take_and_return_ownership(s2.clone()); // must clone otherwise s2 is dropped
    println!("s3 is now: {s3} and s2 is now: {s2}");

    // alternatively, we can use references
    take_reference(&s3); // function definition must specify reference
    println!("Took ownership of s3 references: however it hasnt been dropped - {s3}");

    // lets talk mutability
    let mut s4 = s3.clone();
    println!("s4 starts as: {s4}");
    take_reference_mut(&mut s4);
    println!("Now weve mutated s4: {s4}");

    let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // invalid because the same value can be changed multiple times
    let r1 = &s;
    let r2 = &s;        //   valid because nothing can be changed
    // let r3 = &mut s; // BIG PROBLEM: other references still in scope
    println!("R1 = {r1}, R2 = {r2}");
    let r3 = &mut s; // no problem because r1 and r2 scope doesnt overlap: r1 & r2 can be dropped
    println!("proof that r3 is valid: {r3}");

    // Slicing it up!
    s.push_str(" world"); // now hello world
    let slice_a = &s[..5]; // 0th index to 5th (exclusive)
    let slice_b = &s[6..]; // 6th index to the end
    let slice_c = &s[..]; // now weve just copied the whole string

    println!("slice_a = {slice_a}, slice_b = {slice_b}, slice_c = {slice_c}");

    // Also can be done on arrays!
    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[..3];
    assert_eq!(slice_a, &[1, 2, 3]);

}

fn take_ownership(some_string: String) {
    println!("I took ownership of the string: {some_string}");
}

fn take_and_return_ownership(some_string: String) -> String {
    println!("I took ownership of the string: {some_string}");
    some_string
}

fn take_reference(some_string: &String) {
    println!("I took ownership of the string: {some_string}");
}

fn take_reference_mut(some_string: &mut String) {
    some_string.push_str(" and I can mutate it")
}
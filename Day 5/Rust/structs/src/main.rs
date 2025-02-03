
#[derive(Debug)]
struct Normal {
    name: String,
    age: u8,
    alive: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {            // Associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let new_normal = Normal{
        name: String::from("Kamron"),
        age: 25,
        alive: true,
    };
    println!("Hello {} who is {} years old. living: {}", new_normal.name, new_normal.age, new_normal.alive);

    let assisted_normal = normal_maker(String::from("Kamron"), 25);
    println!("Hello {} who is {} years old. living: {}", assisted_normal.name, assisted_normal.age, assisted_normal.alive);  

    let copied_normal = Normal {
        name: String::from("NOT Kamron"),
        ..new_normal
    };
    println!("Hello {} who is {} years old. living: {}", copied_normal.name, copied_normal.age, copied_normal.alive);
    println!("Original: Hello {} who is {} years old. living: {}", new_normal.name, new_normal.age, new_normal.alive);
    
    // let problematic_normal_copy = Normal {
    //     ..copied_normal                   // this borrows the original name and makes the original unusable
    // }
    // println!("Problematic: Hello {} who is {} years old. living: {}", problematic_normal_copy.name, problematic_normal_copy.age, problematic_normal_copy.alive);
    // println!("Copy 1: Hello {} who is {} years old. living: {}", copied_normal.name, copied_normal.age, copied_normal.alive);

    // Printing custom data types
    println!("Original struct: {:?}", new_normal);
    println!("Assisted struct: {:#?}", assisted_normal);
    println!("Copied struct: {:?}", copied_normal);

    // debug macro!
    let scale = 4;
    let debug_normal = Normal {
        name: String::from("Somebody"),
        age: dbg!(6*scale),
        alive: true,
    };
    dbg!(&debug_normal);

    // Rectangles
    let rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!("Rectangle: {rectangle:#?}");
    println!("Area: {}", rectangle.area());

    let other_rectangle = Rectangle {
        width: 5,
        height: 10,
    };
    println!("Can the first Rectangle hold the second one? {}", rectangle.can_hold(&other_rectangle));
}

fn normal_maker(name: String, age: u8) -> Normal {
    Normal {
        name, // This works because same name: otherwise id use  "name: name"
        age,
        alive: true,
    }
}
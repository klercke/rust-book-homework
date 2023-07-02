#[derive(Debug)] // this annotation allows us to print the struct using the debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // define the width and height of a rectangle in pixels
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect_tuple = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect_tuple)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2) // remember to pass a reference to the struct so main still owns rect2
    );

    // we can print the struct using the debug trait since we added the annotation in the struct definition
    // this is useful for debugging because it gives us good information, but it is not very pretty
    println!("rect2 is {:?}", rect2);

    // we can also print the struct using the debug trait with pretty output. this takes up more space, but is easier to read
    println!("rect2 is {:#?}", rect2);

    // we can also use the dbg macro, which will take ownership of the expression passed to it and print the file and line number
    // where the macro is called, along with the value of the expression it is fed
    // this goes to stderr instead of stdout
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(width1 * scale),
        height: dbg!(height1 * scale),
    };
    dbg!(rect3);

    // create a new rectangle since we destroyed rect3 with dbg!
    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    // we can call the area method on the new rectangle
    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area_method() // rust will automatically convert this this instance to a reference when calling the method, since that's what the method's signature says it expects
    );

    // we can also call the width method on the new rectangle
    if rect4.width() {
        // here, we write width() with parantheses because we want the method called width, not the field
        println!(
            "The rectangle has a nonzero width; it is {} pixels wide.",
            rect4.width
        ); // here, we do not write width with parantheses because we want the field, not the method
    }

    // create 3 new rectangles to compare
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // compare rect1 to rect2 and rect3 to see if rect1 can hold them
    println!(
        "Can rect1 ({}x{}) hold rect2 ({}x{})? {}",
        rect1.width,
        rect1.height,
        rect2.width,
        rect2.height,
        rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 ({}x{}) hold rect3 ({}x{})? {}",
        rect1.width,
        rect1.height,
        rect3.width,
        rect3.height,
        rect1.can_hold(&rect3)
    );

    // create a square
    let square1 = Rectangle::square(10); // this is creates a new instance of the Rectangle struct using the square associated function
    println!("area of square1: {}", square1.area_method());
    println!( // since square is still a rectangle, we can use the can_hold method to see if rect1 can hold square1
              // ideally, I would like to use a square struct that is a child of the rectangle struct, but I don't know how to do that yet
        "Can rect1 ({}x{}) hold square1 ({}x{})? {}",
        rect1.width,
        rect1.height,
        square1.width,
        square1.height,
        rect1.can_hold(&square1)
    );
}

// this function takes the width and height of a rectangle and returns the area
// this is not ideal because it is not clear that the variables are connected outside the context of this function
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// this function does the same thing as the one above but uses a tuple instead
// this is an improvement because it is clear that the variables are connected when they are passed into the function
// but it is still not ideal because it is not clear what the values in the tuple represent
// it doesn't matter in the case of multiplying width and height,
// but it could be confusing in other cases
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// this function also does the same thing as the one above but uses a struct instead
// this is a better option because it is clear that the variables are connected when they are passed into the function
// and it is clear what the values in the struct represent, both in the context of the function and outside of it
// this is also the most flexible option because it allows us to add more fields to the struct in the future without having to change the function
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height // accessing the fields of a borrowed struct does not move them
}

impl Rectangle { // we can have multiple impl blocks for the same struct, but there's no reason for me to separate these
    // begin with the impl keyword, which is short for implementation, meaning we are implementing the rectangle struct
    // any function in this block is called an "associated function" of the Rectangle struct
    // associated functions that don't take self as a parameter are not methods, but can still be useful, such as a constructor function
    // this function is a method that is defined on the Rectangle struct
    // this is the best option because it takes the solution from above and explicitly associates it with the struct
    // since area formulas for different shapes are different,
    // it is best to define the area method on the Rectangle struct specifically to avoid it being called on, for example, a circle struct
    fn area_method(&self) -> u32 {
        // &self in this case is short for self: &Self. the Self type is an alias for whatever type we are implementing the methods on
        // we can take ownership of self, borrow self immutably (like we did here), or borrow self mutably, just like any other parameter
        self.width * self.height
    }

    fn width(&self) -> bool {
        // this method returns true if the rectangle's width is greater than 0
        // the main use for methods with the same name as fields is as getters, so you can have private fields with public getter methods
        self.width > 0
    }

    // this function will take self as a parameter, but will also take in another rectangle to compare to
    fn can_hold(&self, other: &Rectangle) -> bool {
        // this method returns true if the rectangle can hold another rectangle
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        // this is an associated function that is not a method because it does not take self as a parameter
        // this is a constructor function that returns a square Rectangle with the given size
        Rectangle {
            width: size,
            height: size,
        }
    }
}

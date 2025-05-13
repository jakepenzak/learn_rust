fn main() {
    // Naive Approach
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_naive(width1, height1)
    );

    // Tuple Approach
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Struct Approach
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Prints to stderr, returns ownership
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // Printing struct from adding `Debug` outer attribute
    println!("rect1 is {rect1:#?}"); // Printing struct from adding `Debug` outer attribute
    dbg!(&rect1); // Prints to stderr, whereas println prints to stdout. We want a reference so we use &rect1.
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // Method approach
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(20);
    println!("Square is {square:?}");

    let enum_circle = Shapes::Circle(Circle { radius: 10 });
    let enum_rectangle = Shapes::Rectangle(rect2);

    match_shapes(&enum_circle);
    match_shapes(&enum_rectangle);
}

// Naive
fn area_naive(width: u32, height: u32) -> u32 {
    width * height
}

// Tuple
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Struct
#[derive(Debug)] // Adding outer attribute to enable debug printing
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        // Typically this is reserved for getters, to control public/private fields/methods
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Enum Stuff
struct Circle {
    radius: u32,
}

enum Shapes {
    Rectangle(Rectangle),
    Circle(Circle),
}

fn match_shapes(shape: &Shapes) {
    match shape {
        Shapes::Circle(c) => println!("It's a circle! Radius: {}", c.radius),
        Shapes::Rectangle(r) => {
            println!("It's a rectangle! Width: {}, Height: {}", r.width, r.height)
        }
    }
}

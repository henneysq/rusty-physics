// Define a struct called Rectangle with
// some geometric properties
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// Define some functions (implementations) for the
// Rectangle object
impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width,
        }
    }

    fn clone(&self) -> Rectangle {
        Rectangle {
            ..*self
        }
    }

    fn set_length(&mut self, width: u32) {
        self.width = width
    }
    
    fn set_width(&mut self, length: u32) {
        self.length = length
    }
}

fn main() {
    // Declare a mutable rectangle object
    let mut rect1 = Rectangle::new(10, 5);
    // Clone it using the rectangle's `clone` function
    let mut rect2 = rect1.clone();

    // Set length or width
    rect1.set_length(20);
    rect2.set_width(20);

    // Print the rectangles
    println!("{:?}", rect1);
    println!("{:?}", rect2);
}
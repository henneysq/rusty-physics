#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

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
    let mut rect1 = Rectangle::new(10, 5);
    let mut rect2 = rect1.clone();
    rect1.set_length(20);
    rect2.set_width(20);
    println!("{:?}", rect1);
    println!("{:?}", rect2);
}
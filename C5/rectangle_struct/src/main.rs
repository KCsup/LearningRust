fn main() {
    let rect = Rectangle {
        length: 3,
        width: 4
    };

    let rect2 = Rectangle {
        length: 1,
        width: 1
    };

    let s = Rectangle::square(4);

    println!("{}", rect.can_fit_inside(&rect2));
    
    // dbg!(&rect); // 'dbg!' macro takes ownership without reference

    println!("{:?}", s);
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_fit_inside(&self, inside: &Rectangle) -> bool {
        inside.width < self.width && inside.length < self.length
    }

    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size
        }
    }
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.length * rect.width
// }


#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool { // method with same name as attribute
        self.width > 5
    }

    fn can_hold(&self, o: &Rect ) -> bool {
        (self.height >= o.height && self.width >= o.width) ||
        (self.width >= o.height && self.height >= o.width)
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// second impl block -> different modules we  can import?
impl Rect {
    fn super_duper() {
        println!("Super DUPER!");
    }
}

fn main () {
    let r1 = Rect {
        width: 4,
        height: 50,
    };

    // dbg!(&r1);
	// println!("{:#?}", r1);
    
    println!("{}", r1.area());

    if r1.width() {
        println!("Success!");
    }

    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect::square(40);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    Rect::super_duper();
}
#[derive(Debug)]
struct Retangle {
    width: u32, 
    height: u32,
}

impl Retangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width = 30;
    let height = 50;

    let rect2 = (30, 50);

    let rect3 = Retangle{
        width: 30,
        height: 50,
    };

    let rect4 = Retangle{
        width: 10,
        height: 30,
    };


    println!("area1 is : {}", get_area1(width, height));

    println!("area2 is : {}", get_area2(rect2));

    println!("rect3 is : {:#?}, area3 is : {}", rect3, get_area3(&rect3));

    println!("rect4 is : {:#?}, area4 is : {}", rect4, rect4.area());
}

fn get_area1(w: u32, h: u32) -> u32 {
    w * h
}

fn get_area2(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn get_area3(rect: &Retangle) -> u32 {
    rect.width * rect.height
}
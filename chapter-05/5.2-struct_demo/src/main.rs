#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    // let w = 30;
    // let l = 50;
    // println!("{}", area1(w, l));

    // let rect = (30, 50);
    // println!("{}", area2(rect));

    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    println!("{}", area3(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
}

fn area1(width: u32, length: u32) -> u32 {
    width * length
}

fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

use image;
mod codes;

fn main() {
    let img = image::open("input.png").unwrap().to_rgba8();
    codes::q13(img);
}

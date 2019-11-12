use ultraviolet_ray::*;

fn main() {
    let w: usize = 400;
    let h: usize = 200;
    let a = (255.9 * -1.0) as u8;
    println!("{}", a);
    let b = u32::from_ne_bytes([a, 255,255,255]);
    let mut image = ImageBuff::create_with_a_color(w,h,b);
    let mut file = ImageFile::new("pixel.png");
    file.add_image_buff(image);
    file.write_rgba();
}
#[link(name = "badmath", kind = "static")]
extern "C" {
    fn bad_add(v1: f32, v2: f32) -> f32;
}
fn main() {
    println!("Hello, world!");
    let result = unsafe { bad_add(2.3, 3.3) };
    println!("float result: {}", result);
}

/// The function containing the script functionality
pub fn run_script(name: String, count: u8) {
    for _ in 0..count {
        println!("Hello {}!", name);
    }
}

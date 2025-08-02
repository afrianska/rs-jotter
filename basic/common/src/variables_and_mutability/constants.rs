use std::u32;

pub fn constants() {
    // CONSTANT
    // Constant is always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

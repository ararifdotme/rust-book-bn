static mut COUNTER: u32 = 0;

/// SAFETY: একই সময়ে একাধিক thread থেকে এটি call করা undefined behavior, তাই
/// আপনাকে *অবশ্যই* নিশ্চিত করতে হবে যে একবারে শুধু একটি thread থেকে এটি call
/// করছেন।
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: এটি শুধু `main`-এর একটিমাত্র thread থেকে call করা হয়।
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

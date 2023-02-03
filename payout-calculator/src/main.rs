use std::io;
use std::{f32};

fn main() {
    println!(">> Addons payout calculator <<\n");
    loop {
        println!("How many downloads? ");
        let mut var1 = String::new();
        io::stdin().read_line(&mut var1).expect("Parsing error");

        println!("What's the eCPM? ");
        let mut var2 = String::new();
        io::stdin().read_line(&mut var2).expect("Parsing error");

        let dl: f32 = var1.trim().parse().ok().expect("Numbers only");
        let cpm: f32 = var2.trim().parse().ok().expect("Numbers only");
        let count: f32 = 1000.0; // Divide the download count by this, 1K default
        let split: f32 = 0.7; // Payments split, 70/30 default
        let answer: i32 = (dl / count * cpm * split) as i32; // Does the calculation
        println!("The monthly payment comes out to: >> ${answer} <<\n");
        println!("-==-==-==-==-")
    }
}

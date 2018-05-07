mod popcnt;
mod sqrt;

fn main() {

    println!("testing 1");
    println!("{} expect 4",popcnt::popcnt(0b1010_1010));
//    println!("{} expect 8",popcnt::popcnt(0b1111_1111));
//    println!("{} expect 4",popcnt::popcnt(0b1111_0000));
//    println!("{} expect 3",popcnt::popcnt(42));


//    println!("testing 2");
//    println!("{} expect 63",popcnt::popcnt(9223372036854775807)); // 2^63-1
//    println!("{} expect 64",popcnt::popcnt(18446744073709551615));// 2^64 -1
//    println!("{} expect ?",popcnt::popcnt(18446744073709551616)); // 2^64

        println!("testing 3");
    println!("{} expect 2",sqrt::sqrt(4.0));
    println!("{} expect 4",sqrt::sqrt(16.0));
//    println!("{} expect 10",sqrt::sqrt(100.0));
//    println!("{} expect 31.62277",sqrt::sqrt(1000.0));

}




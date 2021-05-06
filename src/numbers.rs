// must add "num" crate to Cargo.toml
// can use cargo-edit package to "cargo add ___" packages
use num::traits::Zero;

pub fn num_crate()
{
    // must specify the type as zero() cannot infer the type
    let num: i32 = Zero::zero();
    println!("{:?}", num);
}

// NAN is part of the floating point family
pub fn not_a_number()
{
    // one NAN never equals another NAN
    assert_eq!(f32::NAN == f32::NAN, false);
}

pub fn conversion()
{
    // NOTE: never convert larger numbers to smaller ones as the
    // value will be truncated and thus be changed; it's always
    // best to "promote" a number, and convert UP to a larger int type

    // using this trait gives us some extra functionality for our numbers
    use std::convert::TryInto;

    // try_into
    let a: u8 = 8;
    let b: u32 = 32;

    let comp = a < b.try_into().unwrap();
    println!("Should be TRUE: {:?}", comp);

    // using "as" is another, simple way to convert
    let comp = (a as u32) < b;
    println!("Should be TRUE: {:?}", comp);

    // Floating point comparisons using PartialEq
}

pub fn print_types()
{
    let a = 10;

    println!("{:?}", print_type_of(&a));
    println!("{:?}", a);
    // this is the most readable unless I have to chain a method on the number
    let b: i32 = 20;
    println!("{:?}", print_type_of(&b));
    println!("{:?}", b);
    let c = 30i32;
    println!("{:?}", print_type_of(&c));
    println!("{:?}", c);
    // I like this best if I were to chain methods like .round()
    let d = 30_i32;
    println!("{:?}", print_type_of(&d));
    println!("{:?}", d);
    let e = add(add(a, b), add(c, d));
    println!("( a + b ) + ( c + d ) = {}", e);

    // NOTE: the 20_i32 syntax is new to me. I like it.
    // The syntax especially makes sense when rounding on a raw value
    let rounded = 201.22_f32.round();

    println!("Rounded is {:?}", rounded);

    // this is another way to do it
    let num = 201.22;
    let rounded = (num as f32).round();
    println!("Rounded is {:?}", rounded);
}

pub fn print_notations()
{
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

pub fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>())
}

pub fn add(i: i32, j: i32) -> i32
{
    i + j
}

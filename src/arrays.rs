pub fn run()
{
    println!("Running Arrays Module");

    let arr = [1, 2, 3];
    println!("{:?} of length : {}", arr, arr.len());

    // fill all 3 spots with 1
    // default on this system is i32
    let arr = [1; 3];
    println!("{:?} of length : {}", arr, arr.len());

    // explicit type declaration w/ max u8
    let arr: [u8; 3] = [255; 3];
    println!("{:?} of length : {}", arr, arr.len());
}

// Slices are important because it’s easier to implement traits for slices than arrays.
// Another important use for slices is their ability to act as a view on arrays (and other slices).
pub fn run()
{
    println!("\nRunning slices module\n");

    let s = "Some long string that is really really long.";
    // inclusive
    let string_slice = &s[0..=3]; // Some
    println!("String Slice {:?}", string_slice);

    // exclusive
    let string_slice = &s[0..3]; // Som
    println!("String Slice {:?}", string_slice);

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // inclusive
    let arr_slice = &a[0..=5]; // 0 -> 5
    println!("Array Slice {:?}", arr_slice);

    // exclusive
    let arr_slice = &a[0..5]; // 0 -> 4
    println!("Array Slice {:?}", arr_slice);
}

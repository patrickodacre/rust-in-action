//! Clone and Copy Example
pub fn run()
{
    let k = Knight { id: 1 };

    // Clone, on the other hand, is explicitly called using .clone()
    let _k2 = k.clone();
}

// we can use derive(Copy) or...
// #[derive(Copy)]
struct Knight
{
    id: u32,
}

// we can impl Copy like so.
// Copy is executed implicitly whenever a value that impls copy is moved.
// The behavior of Copy is not overloadable; it is always a simple bit-wise copy;
// therefore, we cannot impl a custom very of Copy.
impl Copy for Knight {}

// we CAN impl Clone, though.
impl Clone for Knight
{
    fn clone(&self) -> Self
    {
        println!("Cloning Knight id: {:?}", self.id);
        Knight { id: self.id }
    }
}

#![allow(dead_code)]
#![allow(unused_imports)]

mod clone_copy;
mod cube_sats;
mod interior_mutability;


fn main()
{
    interior_mutability::run();
    clone_copy::run();
}

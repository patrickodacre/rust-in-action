#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use std::ops::{Add, Sub};

mod arrays;
mod ifelse;
mod iteration;
mod matching;
mod numbers;
mod slices;
mod strings;
mod traits_generics_1;

fn main()
{
    slices::run();
}

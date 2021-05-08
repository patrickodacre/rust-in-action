//! Basic Traits Module
use std::f32::NAN;
use std::fmt;

pub fn run()
{
    let thing = Thing::new(NAN, NAN, ThingType::Big);

    println!("{}", thing);
}

/// Represents a Type of Thing
enum ThingType
{
    Big,
    Small,
}

/// A Thing is something really cool.
struct Thing
{
    one: f32,
    two: f32,
    thing_type: ThingType,
}

impl Thing
{
    fn new(one: f32, two: f32, thing_type: ThingType) -> Self
    {
        Thing {
            one,
            two,
            thing_type,
        }
    }
}

impl fmt::Display for Thing
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(
            f,
            "Thing One :: {}; Thing Two :: {}; ThingType :: {}",
            self.one, self.two, self.thing_type
        )
    }
}

// our Enum doesn't implement the Display trait
// so we have to do that for our impl to work for Thing above
impl fmt::Display for ThingType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            ThingType::Big => write!(f, "{}", "Big!"),
            ThingType::Small => write!(f, "{}", "Small!"),
        }
    }
}

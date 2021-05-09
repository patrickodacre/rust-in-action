use std::rc::Rc;
use std::boxed::Box;
use std::fmt;

pub fn run() {
    do_rc();
}

fn do_rc() {
    let k = Knight {name: String::from("Danny")};
    let k2 = k.clone();
    let rc = Rc::new(k);

    let rc2 = rc.clone();

    println!("rc :: {:p}", rc);
    println!("rc :: {:p}", rc2);
    println!("k2 :: {:p}", k2);

}

#[derive(Debug)]
struct Knight {
    name: String,
}

impl fmt::Pointer for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use `as` to convert to a `*const T`, which implements Pointer, which we can use

        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl Clone for Knight {
    fn clone(&self) -> Self {
        println!("Cloning a knight");
        let name = &self.name[..];
        Knight {name: name.to_string()}
    }
}
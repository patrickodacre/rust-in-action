use std::boxed::Box;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub fn run()
{
    do_rc();
    do_regular_ref();
    do_ref_rc();
}

fn do_rc()
{
    let k = Knight {
        name: String::from("Danny"),
    };
    let k2 = k.clone();
    let rc = Rc::new(k);

    let rc2 = rc.clone();

    println!("rc :: {:p}", rc);
    println!("rc :: {:p}", rc2);
    println!("k2 :: {:p}", k2);
}

#[derive(Debug)]
struct Knight
{
    name: String,
}

impl fmt::Pointer for Knight
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // use `as` to convert to a `*const T`, which implements Pointer, which we can use

        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl Clone for Knight
{
    fn clone(&self) -> Self
    {
        println!("Cloning a knight");
        let name = &self.name[..];
        Knight {
            name: name.to_string(),
        }
    }
}

fn do_regular_ref()
{
    let mut k = Knight {
        name: "Danny".to_string(),
    };
    println!("Knight :: {:?}", k);

    rename_knight(&mut k, "Christian");
    println!("Knight :: {:?}", k);
    rename_knight(&mut k, "Evan");
    println!("Knight :: {:?}", k);
}

fn rename_knight(k: &mut Knight, name: &str)
{
    k.name = name.to_string();
}

fn do_ref_rc()
{
    let k = Knight {
        name: "Danny".to_string(),
    };

    let k = Rc::new(RefCell::new(k));

    println!("Knight :: {:?}", k);

    change_knight(&k, "Christian");
    println!("Knight :: {:?}", k);

    change_knight(&k, "Evan");
    println!("Knight :: {:?}", k);
}

fn change_knight(k: &Rc<RefCell<Knight>>, name: &str)
{
    let mut knight = k.borrow_mut();
    knight.name = name.to_string();
}

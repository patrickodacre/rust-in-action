pub fn do_iteration()
{
    #[derive(Debug)]
    struct Thing
    {
        value: u32,
    }

    impl Thing
    {
        fn new(v: u32) -> Self
        {
            Thing { value: v }
        }
    }

    let collection = [Thing::new(1), Thing::new(2), Thing::new(3)];

    for item in &collection {
        println!("item {:?}", item);
    }

    // iter() returns elements by REFERENCE
    // as it 'borrows' the collection
    for item in collection.iter() {
        println!("item {:?}", item);
        // error message here shows that item is a reference
        // println!("{:?}", item == &item);
    }

    // mut collection needed for iter_mut()
    println!("\niter_mut()\n");
    let mut collection = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for item in collection.iter_mut() {
        *item = *item + 1;
        println!("item {}", item);
    }

    // into_iter() returns elements by VALUE
    // as it 'takes ownership' of the collection
    println!("\ninto_iter()\n");

    // we don't HAVE TO use into_iter() for a vec
    let collection = vec![Thing::new(1), Thing::new(2), Thing::new(3)];

    for item in collection.into_iter() {
        println!("item {:?}", item);
        // error here shows that item is a value and not a reference
        // println!("{:?}", item == &item);
    }
}

pub fn do_forever()
{
    // not idiomatic Rust
    // while true {}

    // idiomatic Rust
    loop {}
}

pub fn do_iteration_2()
{
    let mut i = 0;

    // exclusive
    for _ in 0..10 {
        println!("{:?}", i);
        i = i + 1;
    }

    let mut i = 0;

    // inclusive
    for _ in 0..=10 {
        println!("{:?}", i);
        i = i + 1;
    }
}

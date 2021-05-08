pub fn run()
{
    // the new-type pattern
    // for when you want the compiler to treat
    // your type as a distinct type,
    // not just an alias
    #[derive(Debug, PartialEq)]
    struct Hostname(String);

    let hn = Hostname(String::from("localhost"));

    println!("{:?}", hn);
}

pub fn do_matching()
{
    let collection = vec![1, 2, 3, 4, 5];
    for item in &collection {
        println!("item {:?}", item);

        match item {
            1 => {
                println!("First item!");
            }
            2..=4 => {
                println!("Middle items...");
            }
            5 => {
                println!("Last item.");
                println!("We're done!");
            }
            _ => {
                println!("We have to handle all cases, so a default is needed");
            }
        }
    }
}

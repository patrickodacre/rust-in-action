use std::ops::Deref;

fn main()
{
    let mut root = Node::new(20);
    println!("root :: {:?}", root);

    let mut child_1 = Node::new(10);
    let child_2 = Node::new(5);

    let _res = root.add_child(&child_1);
    let _res = child_1.add_child(&child_2);
}

#[derive(Debug)]
struct Node<'node_lt>
{
    value: i32,
    children: Vec<&'node_lt Node<'node_lt>>,
}

impl<'node_lt> Node<'node_lt>
{
    fn new(value: i32) -> Self
    {
        Self {
            value,
            children: Vec::<&'node_lt Node<'node_lt>>::new(),
        }
    }
    fn add_child(&mut self, n: &'node_lt Node) -> Result<(), &'static str>
    {
        self.children.push(n);
        Ok(())
    }
}

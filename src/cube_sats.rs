//! Four general strategies can help with ownership issues:
//!
//! - Use references where full ownership is not required
//! - Duplicate the value
//! - Refactoring code to reduce the number of long-lived objects
//! - Wrap your data in a type designed to assist with movement issues
#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage
{
    Ok,
}

#[derive(Debug)]
struct CubeSat
{
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat
{
    fn new(id: u64) -> Self
    {
        CubeSat {
            id,
            mailbox: Mailbox {
                messages: Vec::<Message>::new(),
            },
        }
    }

    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message>
    {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox
{
    messages: Vec<Message>,
}

impl Mailbox
{
    fn new() -> Self {
        Mailbox {messages: Vec::<Message>::new()}
    }

    fn post(&mut self, message: Message)
    {
        self.messages.push(message);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>
    {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message
{
    to: u64,
    body: String,
}

struct GroundStation;

impl GroundStation
{
    /// send a message to a CubeSat
    /// it's ok for this function to take ownership of the message
    fn send(mailbox: &mut Mailbox, msg: Message)
    {
        mailbox.post(msg)
    }
}

pub fn run()
{
    let mut sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let a_status = check_status(&sat_a);
    let b_status = check_status(&sat_b);
    let c_status = check_status(&sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // "waiting" ...
    let a_status = check_status(&sat_a);
    let b_status = check_status(&sat_b);
    let c_status = check_status(&sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let mut mailbox = Mailbox::new();

    let msg = Message {to: sat_a.id, body: "Some Message".to_string()};

    // I don't need the msg after this, so I can give ownership to the send() function
    GroundStation::send(&mut mailbox, msg);

    let msg = sat_a.recv(&mut mailbox);

    println!("msg received {:?}", msg);
}

fn check_status(cube_sat: &CubeSat) -> StatusMessage
{
    StatusMessage::Ok
}

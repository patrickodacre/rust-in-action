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

    fn recv(&mut self) -> Option<String>
    {
        let message = self.mailbox.messages.pop().unwrap_or("".to_string());

        Some(message)
    }
}

#[derive(Debug)]
struct Mailbox
{
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation
{
    /// send a message to a CubeSat
    /// it's ok for this function to take ownership of the message
    fn send(sat: &mut CubeSat, msg: Message)
    {
        sat.mailbox.messages.push(msg);
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

    GroundStation::send(&mut sat_a, "Some message".to_string());
    println!("Got a message {:?}", sat_a.recv());

    GroundStation::send(&mut sat_a, "This is a great message".to_string());
    println!("Got a message {:?}", sat_a.recv());
}

fn check_status(cube_sat: &CubeSat) -> StatusMessage
{
    StatusMessage::Ok
}

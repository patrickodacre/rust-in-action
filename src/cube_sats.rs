//! Four general strategies can help with ownership issues:
//!
//! - Use references where full ownership is not required
//! - Duplicate the value
//! - Refactoring code to reduce the number of long-lived objects
//! - Wrap your data in a type designed to assist with movement issues
#![allow(unused_variables)]

#[derive(Debug)]
enum ConnectionError {
    HostRefusedConnection
}

#[derive(Debug)]
struct CubeSat
{
    id: u64,
}

impl CubeSat
{
    fn new(id: u64) -> Self
    {
        CubeSat { id }
    }

    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message>
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
    fn send(&self, mailbox: &mut Mailbox, msg: Message)
    {
        mailbox.post(msg)
    }

    fn connect(&self, id: u64) -> Result<CubeSat, ConnectionError>
    {
        if id == 0 {
            return Err(ConnectionError::HostRefusedConnection)
        }

        Ok(CubeSat::new(id))
    }
}

fn get_sat_ids() -> Vec::<u64>
{
    vec![0,1,2]
}

pub fn run()
{
    let mut mailbox = Mailbox::new();
    let base = GroundStation {};

    // we could be getting these ids from a database 
    let sat_ids = get_sat_ids();

    for item in sat_ids {

        // this is mimicing a connection where, presumably, some verification, etc. is done
        match base.connect(item) {
            Ok(sat) => {

                let msg = Message {to: sat.id, body: format!("Some Message to {}", item)};

                // I don't need the msg after this, so I can give ownership to the send() function
                base.send(&mut mailbox, msg)
            }
            Err(err) => {
                println!("Could not connect to sat {}", item);
            }
        }
    }

    if let Ok(sat) = base.connect(1) {
        let msg = sat.recv(&mut mailbox).unwrap();

        println!("msg received {:?}", msg);
    }

    if let Ok(sat) = base.connect(2) {
        let msg = sat.recv(&mut mailbox).unwrap();

        println!("msg received {:?}", msg);
    }
}

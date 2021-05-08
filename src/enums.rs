// Enums can be a powerful aide in your quest to produce reliable, robust software.
// Consider it for your code whenever you discover yourself introducing stringly typed data such as messages codes.
pub fn run() {
    emit_event(Event::Create, "This this is created");

    let k = Knight::White;
    k.hail();

    let k = Knight::Black;
    k.hail();
}

fn emit_event(evt: Event, msg: &str) {
    match evt {
        Event::Create => {
            println!("Something Created!");
        }
        Event::Read => {
            println!("Something Read...");
        }
        Event::Update => {
            println!("Something Updated!");
        }
        Event::Delete => {
            println!("Something deleted...");
        }
        _ => {
            println!("Unknown event.");
        }
    }
}

#[derive(Debug)]
enum Knight {
    White,
    Black,
    Gray,
}

// you can impl an enum
impl Knight {
    fn hail(&self) {
        println!("Hail! I am the {:?} Knight.", self);
    }
}

enum Event {
    Create,
    Read,
    Update,
    Delete,
    Unknown,
}

use rand::random;
use std::str;

pub fn run() {
    println!("Running Errors Module");

    let mut f = File::new("MyFile.txt", "Hello, World!");
    println!("File State:: {:?}", f.state);
    println!("F Data:: {:?}", f.data);

    let mut buffer = Vec::<u8>::new();
    match f.open(&mut buffer) {
        Ok(_) => {}
        Err(err) => {
            println!("Error :: {:?}", err);
        }
    }

    println!("File State:: {:?}", f.state);
    match f.close() {
        Ok(_) => {}
        Err(err) => {
            println!("Error :: {:?}", err);
        }
    }

    println!("File State:: {:?}", f.state);
    println!("Buffer:: {:?}", buffer);
}

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

#[derive(Debug)]
enum FileError {
    FileNotFound,
    FailedToClose,
}

impl File {
    fn new(name: &str, data: &str) -> Self {
        File {
            name: name.to_string(),
            data: data.as_bytes().to_vec(),
            state: FileState::Closed,
        }
    }

    fn open(&mut self, buffer: &mut Vec<u8>) -> Result<(), FileError> {
        self.state = FileState::Open;

        // randomly fail to open the "file"
        if random() {
            return Err(FileError::FileNotFound);
        }

        for item in self.data.iter() {
            buffer.push(*item);
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), FileError> {
        if random() {
            return Err(FileError::FailedToClose);
        }

        self.state = FileState::Closed;

        Ok(())
    }
}

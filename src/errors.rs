use rand::random;
use std::str;

pub fn run() {
    println!("Running Errors Module");

    let f = File::new("MyFile.txt", "Hello, World!");
    println!("F Data:: {:?}", f.data);

    let mut buffer = Vec::<u8>::new();
    match f.open(&mut buffer) {
        Ok(_) => {}
        Err(err) => {
            println!("Error :: {:?}", err);
        }
    }

    println!("Buffer:: {:?}", buffer);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug)]
enum FileError {
    FileNotFound,
}

impl File {
    fn new(name: &str, data: &str) -> Self {
        File {
            name: name.to_string(),
            data: data.as_bytes().to_vec(),
        }
    }

    fn open(&self, buffer: &mut Vec<u8>) -> Result<(), FileError> {
        // randomly fail to open the "file"
        if random() {
            return Err(FileError::FileNotFound);
        }

        for item in self.data.iter() {
            buffer.push(*item);
        }

        Ok(())
    }
}

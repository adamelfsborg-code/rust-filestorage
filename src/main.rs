use std::collections::HashMap;
use std::path::Path;

const FILE_NAME: &str = "kv.db";

fn main() {
    let mut arguments = std::env::args().skip(1);
    
    let action = arguments.next().expect("Action not passed");
    let mut database = Database::new().unwrap();

    match action.as_str() {
        "insert" => {
            let key = arguments.next().expect("Key not passed");
            let value = arguments.next().expect("Value not passed");
            database.insert(key, value)
        },
        "get" => {
            let key = arguments.next().expect("Key not passed");
            database.get(key)
        },
        "delete" => {
            let key = arguments.next().expect("Key not passed");
            database.delete(key)
        }
        &_ => todo!()
    }
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        let file_not_exists = Path::new(FILE_NAME).exists();

        if file_not_exists != true {
            return Ok(Database { map })
        }

        let contents = std::fs::read_to_string(FILE_NAME)?;
        for line in contents.lines() {
            let mut chunk = line.splitn(2, '\t');
            let key = chunk.next().expect("Key not found");
            let value = chunk.next().expect("Value not found");

            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: String) {
        let value = self.map.get(&key);
        println!("{:?}", value)
    }

    fn delete(&mut self, key: String) {
        self.map.remove(key.as_str());
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (key, val) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(val);
            contents.push('\n');
        }
        std::fs::write(FILE_NAME, contents)
    }

}


impl Drop for Database {
    fn drop(&mut self) {
        self.flush().unwrap()
    }
}
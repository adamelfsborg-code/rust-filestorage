use std::collections::HashMap;
use std::path::Path;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key not found");
    let value = arguments.next().expect("Value not found");
    
    let mut database = Database::new().unwrap();
    database.insert(key, value.clone());
    database.flush().expect("Failed to flush database");
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        let file_not_exists = Path::new("kv.db").exists();

        if file_not_exists != true {
            return Ok(Database { map })
        }

        let contents = std::fs::read_to_string("kv.db")?;
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

    fn flush(self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (key, val) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(val);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }

}
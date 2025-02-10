
pub mod db {
  
    use std::{any::Any, fmt::Debug, mem};
    use std::io::{self, BufWriter, Write};
    use std::fs::File;
    
    #[derive(Debug)]
    pub struct data_block {
        key: &'static str,
        value: &'static str,
    }
    #[derive(Debug)]
    pub struct db {
        id: &'static str,
        data: [data_block; 50],
    }

    impl std::fmt::Display for data_block {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Key: {} Value {}", self.key, self.value)
        }
    }

    impl std::fmt::Display for db {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,  "id: {} data: {:?}", self.id, self.data)
        }
    }

    impl data_block {
        pub fn new(&self, key: &'static str, value: &'static str) -> data_block {
            Self {key: key, value: value}
        }
    }
    impl db {
        pub fn new(&self, id: &'static str, data: [data_block; 50]) -> db {
            Self {id: id, data: data}
        }
        pub fn write_out(&self, path: &str) {
            let mut file = File::create(path).expect("invalid file path");
            let mut writer = BufWriter::new(file);
            for i in 0..mem::size_of_val(&self.data) - 1 {
                writeln!(writer, "Key: {} Value: {}", self.data[i].key, self.data[i].value).expect("err writing to file");

            }
            
            
            
        }
    }
    pub fn db_add_data(mut db: db, datablock: data_block)  {
        let size = mem::size_of_val(&db.data);
        db.data[size] = datablock;

    }
    pub fn db_empty(mut db: db) {
        for i in 0..db.data.len() - 1 {
            db.data[i].key = "";
            db.data[i].value = "";
        }
        println!("new db: {}", db);
    }

}


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
        data: [Option<data_block>; 50],
        current_pos: usize,
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
        pub fn new(key: &'static str, value: &'static str) -> data_block {
            Self {key: key, value: value}
        }
        
    }
    impl db {
        pub fn new(id: &'static str) -> db {
            
            Self {id: id , data: std::array::from_fn(|_| None ), current_pos: 0}
        }
        pub fn write_out(&self, path: &str) {
            let mut file = File::create(path).expect("invalid file path");
            let mut writer = BufWriter::new(file);
            for i in 0..self.data.len() {
                if let Some(data_block) = &self.data[i] {
                    writeln!(writer, "Key: {} Value: {}", data_block.key, data_block.value)
                        .expect("err writing to file");
                }
            }
        }
    }
    pub fn db_add_data(db: &mut db, datablock: data_block)  {
        if db.current_pos < db.data.len() {
            db.data[db.current_pos]  = Some(datablock);
            db.current_pos += 1;
        } else {
            println!("DB full");
        }
        
        

    }
    pub fn db_read(db: &db, key: &'static str) -> Option<&'static str> {
        for i in 0..db.data.len() {
            if let Some(data_block) = &db.data[i] {
                if data_block.key == key {
                    return Some(data_block.value); 
                }
            }
        }
        None 
    }
    pub fn db_empty(mut db: db) {
        for i in 0..db.data.len() - 1 {
            if let Some(data_block) = &mut db.data[i] {
                data_block.key = "";
                data_block.value = ""; 
            }
        }
        println!("new db: {}", db);
    }

}

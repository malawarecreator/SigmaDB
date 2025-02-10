
pub mod db {
    use std::{any::Any, fmt::Debug, mem};
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
        fn new(&self, key: &'static str, value: &'static str) -> data_block {
            Self {key: key, value: value}
        }
    }
    impl db {
        fn new(&self, id: &'static str, data: [data_block; 50]) -> db {
            Self {id: id, data: data}
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

#![allow(dead_code)]
pub struct Items {
    id: i64,
    qty: i64,
    name: String
}

impl Items {
    pub fn new(a: i64, b: i64, c: String) -> Items {
        Items {
            id: a,
            qty: b,
            name: c
        }
    }
    pub fn get_id(&self) -> i64{
        self.id
    }
}
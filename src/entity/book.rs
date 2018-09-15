use super::*;
use slotmap::Key;

pub struct Book {
    pub title: String,

    pub key: Key,
    pub owner_key: Key,
}

impl Book {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            title: name.into(),

            key: Key::null(),
            owner_key: Key::null(),
        }
    }
}

impl OverloadedParentKey<Person> for Book {
    fn overloaded_set_parent_key(&mut self, parent_key: Key) {
        self.owner_key = parent_key;
    }
}

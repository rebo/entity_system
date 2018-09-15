use slotmap::Key;

pub mod book;
pub mod person;

pub use self::book::Book;
pub use self::person::Person;

pub trait Entity {
    fn key(&self) -> Key;
    fn set_key(&mut self, key: Key) -> Key;

    fn child_keys<T>(&mut self) -> &mut Vec<Key>
    where
        Self: OverloadedChildKeys<T>,
    {
        self.overloaded_child_keys()
    }

    fn set_parent_key<T>(&mut self, parent_key: Key)
    where
        Self: OverloadedParentKey<T>,
    {
        self.overloaded_set_parent_key(parent_key)
    }

    fn is_stored(&self) -> bool {
        !self.key().is_null()
    }
}

impl Entity for Person {
    fn key(&self) -> Key {
        self.key
    }

    fn set_key(&mut self, key: Key) -> Key {
        self.key = key;
        key
    }
}

impl Entity for Book {
    fn key(&self) -> Key {
        self.key
    }

    fn set_key(&mut self, key: Key) -> Key {
        self.key = key;
        key
    }
}
pub trait OverloadedChildKeys<T> {
    fn overloaded_child_keys<'a>(&'a mut self) -> &'a mut Vec<Key>;
}

pub trait OverloadedParentKey<T> {
    fn overloaded_set_parent_key(&mut self, parent_key: Key);
}

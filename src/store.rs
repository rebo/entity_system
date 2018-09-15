// External Uses

use anymap::any::Any;
use slotmap::{Key, SlotMap};

// Std Uses
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub type SMReadResult<'a, E> =
    Result<RwLockReadGuard<'a, SlotMap<E>>, PoisonError<RwLockReadGuard<'a, SlotMap<E>>>>;

pub type SMWriteResult<'a, E> =
    Result<RwLockWriteGuard<'a, SlotMap<E>>, PoisonError<RwLockWriteGuard<'a, SlotMap<E>>>>;

// local crate uses
use crate::domain::*;
use crate::entity::*;

// help type definitions
// This allows us to more neatly return a RwLockRead and WriteGuard

// The only field is an empyy anymap
#[derive(Default)]
pub struct Store {
    pub anymap: anymap::Map<Any + Sync + Send>,
}

impl Store {
    pub fn new() -> Store {
        let mut store = Store {
            anymap: anymap::Map::new(),
        };
        store.register_components();
        store
    }

    pub fn store_entity<T: Entity + Send + Sync + 'static>(&self, entity: T) -> Key {
        match self.write_to::<T>() {
            Ok(mut sm) => sm.store_entity(entity),
            Err(_err) => Key::null(),
        }
    }

    // various read_froms to read from multiple slotmaps at a time
    pub fn read_from<E: Entity + Send + Sync + 'static>(
        &self,
    ) -> Result<RwLockReadGuard<SlotMap<E>>, PoisonError<RwLockReadGuard<SlotMap<E>>>> {
        self.get_entity_slotmap::<E>().read()
    }

    pub fn read_from2<E: Entity + Send + Sync + 'static, F: Entity + Send + Sync + 'static>(
        &self,
    ) -> (SMReadResult<E>, SMReadResult<F>) {
        (
            self.get_entity_slotmap::<E>().read(),
            self.get_entity_slotmap::<F>().read(),
        )
    }

    pub fn read_from3<
        E: Entity + Send + Sync + 'static,
        F: Entity + Send + Sync + 'static,
        G: Entity + Send + Sync + 'static,
    >(
        &self,
    ) -> (SMReadResult<E>, SMReadResult<F>, SMReadResult<G>) {
        (
            self.get_entity_slotmap::<E>().read(),
            self.get_entity_slotmap::<F>().read(),
            self.get_entity_slotmap::<G>().read(),
        )
    }

    // write to a single slotmap at a time
    pub fn write_to<E: Entity + Send + Sync + 'static>(&self) -> SMWriteResult<E> {
        self.get_entity_slotmap::<E>().write()
    }
    //  check whether an entity exists

    pub fn entity_exists<E: Entity + Send + Sync + 'static>(&self, key: Key) -> bool {
        if let Ok(sm) = self.read_from::<E>() {
            sm.contains_key(key)
        } else {
            false
        }
    }

    pub fn store_parent_child_keys<
        P: Entity + Send + Sync + OverloadedChildKeys<C> + 'static,
        C: Entity + Send + Sync + OverloadedParentKey<P> + 'static,
    >(
        &self,
        parent_key: Key,
        child_key: Key,
    ) -> (Key, Key) {
        let parent_exists = self.entity_exists::<P>(parent_key);
        let child_exists = self.entity_exists::<C>(child_key);

        if parent_exists && child_exists {
            if let Ok(mut p_sm) = self.write_to::<P>() {
                let parent = p_sm.get_mut(parent_key).unwrap();
                parent.child_keys().push(child_key);
                println!("ck {:#?}", parent.child_keys());
            };

            if let Ok(mut c_sm) = self.write_to::<C>() {
                let child = c_sm.get_mut(child_key).unwrap();
                child.set_parent_key(parent_key);
            };

            (parent_key, child_key)
        } else {
            (Key::null(), Key::null())
        }
    }

    pub fn delete_entity<T: Entity + Send + Sync + 'static>(&self, key: Key) -> Option<T> {
        let sm = self.get_entity_slotmap::<T>().write();
        if let Ok(mut sm) = sm {
            sm.remove(key)
        } else {
            None
        }
    }

    // private functions that make the store work

    pub fn register_component<T: Send + Sync + 'static>(&mut self) {
        let sm = SlotMap::<T>::new();
        let rw = RwLock::new(sm);

        self.anymap.insert(rw);
    }

    pub fn register_components(&mut self)
    where
        Self: RegisterComponents,
    {
        self.domain_register_components()
    }

    // With this method The application is responsible for ensuring that an entity exists
    fn get_entity_slotmap<T: Entity + Send + Sync + 'static>(&self) -> &RwLock<SlotMap<T>> {
        self.anymap
            .get::<RwLock<SlotMap<T>>>()
            .expect("Entity Slotmap doesn't exist")
    }
}

// Add a method to SlotMap to ensure an enttity gets added with its own key
pub trait StorageSupport<T> {
    fn store_entity(&mut self, entity: T) -> Key;
}

impl<T: Entity + Send + Sync + 'static> StorageSupport<T> for SlotMap<T> {
    fn store_entity(&mut self, mut entity: T) -> Key {
        self.insert_with_key(|k| {
            entity.set_key(k);
            entity
        })
    }
}

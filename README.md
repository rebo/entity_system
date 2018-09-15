# Entity System

A Naive attempt at an entity application architecture based on recent Rustconf talk (Without E**C**S, just Entities and System).

This includes an AnyMap/RwLock/SlotMap backed store. I.e.

```rust
pub struct Store {
    pub anymap: anymap::Map<Any + Sync + Send>,
}
```

and:

```rust
///...
pub fn register_component<T: Send + Sync + 'static>(&mut self) {
        let sm = SlotMap::<T>::new();
        let rw = RwLock::new(sm);

        self.anymap.insert(rw);
    }
///...

impl RegisterComponents for Store {
    fn domain_register_components(&mut self) {
        self.register_component::<Person>();
        self.register_component::<Book>();
    }
}
```

The above snippets have 'components' of Person and Book. These are not components in the ECS sense they are just ordinary objects. However they could  just as easily be ECS style components i.e. `self.register_component::<Health>();` would be just as valid.

The 'Store' part of the logic is isolated from the business logic (which resides in /entity folder and domain.rs).

Mutation of state is protected by RwLock, granular on specific SlotMaps.  The best way to scope access is via if Let and match. i.e.

```rust
let person = Person::new("The Hulk");

let p_key = STORE.store_entity(person);

// gives access to the 'person' slotmap in read only form.
if let Ok(person_slotmap) = STORE.read_from::<Person>() {
    let person = person_slotmap.get(p_key).expect("person doesnt exist!");

    assert_eq!(person.name, "The Hulk");
}
```

Anyway this is just a proof of concept and not actually proper code as I still learning.






# Autowired

![crates.io](https://img.shields.io/crates/v/autowired.svg)
![docs.rs](https://docs.rs/autowired/badge.svg)

Rust dependency injection project

## Usage

Just derive your struct with the marco `Component`, you can use the singleton component everywhere.

```rust
#[derive(Default, Component)]
struct Bar {
    name: String,
    age: u32,
}

fn main() {
    // create `bar` via Default::default
    let bar: Autowired<Bar> = Autowired::new();

    assert_eq!(String::default(), bar.name);
    assert_eq!(u32::default(), bar.age);
}
```

Define custom component initialization logic

```rust
#[derive(Default)]
struct Foo {
    value: String,
}

impl Component for Foo {
    fn new_instance() -> Result<Arc<Self>, Box<dyn Error>> {
        Ok(Arc::new(Foo {
            value: "TEST_STRING".to_string(),
        }))
    }
}

fn main() {
    let foo = Autowired::<Foo>::new();

    assert_eq!("TEST_STRING", foo.value);
}
```

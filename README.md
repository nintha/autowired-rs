# Autowired

[![crates.io](https://img.shields.io/crates/v/autowired.svg)](https://crates.io/crates/autowired)
[![docs.rs](https://docs.rs/autowired/badge.svg)](https://docs.rs/autowired)

Rust dependency injection project, inspired by `Spring IOC`.

## Add Dependency

```toml
[dependencies]
autowired="0.1"
```

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
    type Error = ();

    fn new_instance() -> Result<Arc<Self>, Self::Error> {
        Ok(Arc::new(Foo {
            value: TEST_STRING.to_string(),
        }))
    }
}

fn main() {
    // create `foo` via new_instance
    let foo = Autowired::<Foo>::new();

    assert_eq!("TEST_STRING", foo.value);
}
```

## Central registration in the beginning of the program

By default, components are registered lazily. 
If you need to register components in advance at the beginning of the program, 
you can refer to this example:

```rust
use autowired::{Component, Bean, setup_submitted_beans};

#[derive(Default, Component, Bean)]
struct Foo;

#[derive(Default, Component, Bean)]
struct Bar;

fn main() {
    // register components which derives `Bean`
    setup_submitted_beans();

    assert!(autowired::exist_component::<Foo>());
    assert!(autowired::exist_component::<Bar>());
}
```

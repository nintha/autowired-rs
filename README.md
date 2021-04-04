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
    // central registration in the beginning of the program
    setup_submitted_beans();

    // create `bar` via Default::default
    let bar: Autowired<Bar> = Autowired::new();

    assert_eq!(String::default(), bar.name);
    assert_eq!(u32::default(), bar.age);
}
```

Define custom component initialization logic

```rust
struct Goo { pub list: Vec<String> }

#[autowired::bean]
fn build_goo() -> Goo {
    Goo { list: vec!["hello".to_string()] }
}

fn main() {
    // central registration in the beginning of the program
    setup_submitted_beans();

    let goo = Autowired::<Goo>::new();
    assert_eq!("hello", goo.list[0])
}
```

## Lazy components

By default, components are registered with `setup_submitted_beans`. 
If you need to register components lazily, you can refer to this example:

```rust
use std::sync::Arc;
use autowired::{ LazyComponent, setup_submitted_beans, bean, Autowired};

#[allow(dead_code)]
#[derive(Default, LazyComponent)]
struct Bar {
    name: Arc<String>,
    age: u32,
}

#[allow(dead_code)]
struct Goo { pub list: Vec<String> }

#[bean(lazy)]
fn build_goo() -> Goo {
    Goo { list: vec!["hello".to_string()] }
}

#[test]
fn lazy() {
    setup_submitted_beans();

    assert!(!autowired::exist_component::<Bar>());
    assert!(!autowired::exist_component::<Goo>());

    let bar = Autowired::<Bar>::new();
    assert!( bar.name.is_empty());

    let goo = Autowired::<Goo>::new();
    assert_eq!("hello", goo.list[0]);

    assert!(autowired::exist_component::<Bar>());
    assert!(autowired::exist_component::<Goo>());
}
```

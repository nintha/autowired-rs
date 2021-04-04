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



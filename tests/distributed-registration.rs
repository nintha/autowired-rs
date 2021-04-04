use std::sync::Arc;
use autowired::{Component, setup_submitted_beans, Autowired};

#[allow(dead_code)]
#[derive(Default, Component)]
struct Bar {
    name: Arc<String>,
    age: u32,
}

#[allow(dead_code)]
struct Goo { pub list: Vec<String> }

#[autowired::bean]
fn build_goo() -> Goo {
    Goo { list: vec!["hello".to_string()] }
}

#[test]
fn distributed_registration() {
    setup_submitted_beans();

    assert!(autowired::exist_component::<Bar>());
    assert!(autowired::exist_component::<Goo>());

    let goo = Autowired::<Goo>::new();
    assert_eq!("hello", goo.list[0])
}



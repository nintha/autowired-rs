use autowired::{setup_submitted_beans, bean, Autowired};

#[allow(dead_code)]
struct Bar {
    name: String,
}

#[bean(option)]
fn build_bar_none() -> Option<Bar> {
    None
}

#[allow(dead_code)]
struct Goo {
    pub list: Vec<String>,
}

#[bean(option)]
fn build_goo_some() -> Option<Goo> {
    Some(Goo { list: vec!["hello".to_string()] })
}

#[allow(dead_code)]
struct Foo {
    pub list: Vec<String>,
}

#[bean(option, lazy)]
fn build_foo() -> Option<Foo> {
    Some(Foo { list: vec!["world".to_string()] })
}

#[test]
fn option() {
    setup_submitted_beans();

    assert!(!autowired::exist_component::<Bar>());
    assert!(autowired::exist_component::<Goo>());
    assert!(!autowired::exist_component::<Foo>());

    let goo = Autowired::<Goo>::new();
    assert_eq!("hello", goo.list[0]);

    let foo = Autowired::<Foo>::new();
    assert_eq!("world", foo.list[0]);

    assert!(!autowired::exist_component::<Bar>());
    assert!(autowired::exist_component::<Goo>());
    assert!(autowired::exist_component::<Foo>());
}



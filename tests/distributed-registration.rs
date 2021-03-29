use std::sync::Arc;
use autowired::{Component, Bean, setup_submitted_beans, bean};

#[allow(dead_code)]
#[derive(Bean)]
struct Foo {
    value: String,
}

impl Component for Foo {
    type Error = ();

    fn new_instance() -> Result<Arc<Self>, Self::Error> {
        Ok(Arc::new(Foo {
            value: "TEST_STRING".to_string(),
        }))
    }
}

#[allow(dead_code)]
#[derive(Default, Component)]
struct Bar {
    name: Arc<String>,
    age: u32,
}

#[bean]
fn build_bar() -> Bar {
    Bar::default()
}

#[test]
fn distributed_registration() {
    setup_submitted_beans();

    assert!(autowired::exist_component::<Foo>());
    assert!(autowired::exist_component::<Bar>());
}



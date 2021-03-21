use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use once_cell::sync::OnceCell;
use autowired::{Component, Autowired};


const TEST_STRING: &str = "1234567890";

fn atomic_count() -> &'static AtomicU32 {
    static INSTANCE: OnceCell<AtomicU32> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

#[derive(Default)]
struct Foo {
    value: String,
}

impl Component for Foo {
    type Error = anyhow::Error;

    fn new_instance() -> Result<Arc<Self>, Self::Error> {
        let foo = Arc::new(Foo {
            value: TEST_STRING.to_string(),
        });
        atomic_count().fetch_add(1, Ordering::SeqCst);
        Ok(foo)
    }
}

#[derive(Default, Component)]
struct Bar {
    name: String,
    age: u32,
}

#[test]
fn register_foo() {
    assert_eq!(0, atomic_count().load(Ordering::SeqCst));

    let foo = Autowired::<Foo>::new();

    assert_eq!(TEST_STRING, foo.value);
    assert_eq!(1, atomic_count().load(Ordering::SeqCst));
}

#[test]
fn register_bar() {
    let bar: Autowired<Bar> = Autowired::new();

    assert_eq!(String::default(), bar.name);
    assert_eq!(u32::default(), bar.age);
}


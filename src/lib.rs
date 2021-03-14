use std::sync::Arc;
use dashmap::DashMap;
use std::any::Any;
use std::any::type_name;
use std::ops::Deref;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::error::Error;
pub use autowired_derive::Component;

fn component_mutex() -> &'static Mutex<u64> {
    static INSTANCE: OnceCell<Mutex<u64>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn component_dashmap() -> &'static DashMap<String, Arc<dyn Any + 'static + Send + Sync>> {
    static INSTANCE: OnceCell<DashMap<String, Arc<dyn Any + 'static + Send + Sync>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn get_component<T: Component>() -> Option<Arc<T>> {
    component_dashmap().get(type_name::<T>())
        .map(|x| x.value().clone())
        .map(|x| x.downcast::<T>().ok())
        .flatten()
}

/// return true if component exists
pub fn exist_component<T: Component>() -> bool {
    component_dashmap().contains_key(type_name::<T>())
}

pub trait Component: Any + 'static + Send + Sync {
    /// create a new component instance
    fn new_instance() -> Result<Arc<Self>, Box<dyn Error>>;

    /// call `new_instance` to create new component, then add it into a global map
    fn register() where Self: std::marker::Sized {
        let name = type_name::<Self>();
        // 在注册组件的时候进行加锁，防止出现多次初始化
        if let Ok(mut count) = component_mutex().lock() {
            if component_dashmap().contains_key(name) {
                return;
            }

            let component: Arc<Self> = match Self::new_instance() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("[Component] register failure, {}", e);
                    return;
                }
            };
            component_dashmap().insert(name.to_string(), component.clone());
            *count += 1;

            log::debug!("[Component] register, name={}", name);
            component.after_register();
        }
    }

    /// run code after component register
    fn after_register(&self) {}
}

/// lazy autowired
pub struct Autowired<T> {
    inner: OnceCell<Arc<T>>,
}

impl<T> Autowired<T> {
    pub const fn new() -> Self {
        Autowired { inner: OnceCell::new() }
    }
}

impl<T: Component> Deref for Autowired<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| {
            if !exist_component::<T>() {
                T::register()
            }
            get_component::<T>().unwrap_or_else(||
                panic!(format!("[Autowired] not found component {}", type_name::<T>()))
            )
        })
    }
}

impl<T: Component> Default for Autowired<T> {
    fn default() -> Self {
        Autowired::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Component, Autowired};
    use std::sync::Arc;
    use std::error::Error;
    use std::sync::atomic::{AtomicU32, Ordering};
    use once_cell::sync::OnceCell;

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
        fn new_instance() -> Result<Arc<Self>, Box<dyn Error>> {
            Ok(Arc::new(Foo {
                value: TEST_STRING.to_string(),
            }))
        }
        fn after_register(&self) {
            atomic_count().fetch_add(1, Ordering::SeqCst);
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
}

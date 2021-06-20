use std::sync::Arc;
use dashmap::DashMap;
use std::any::Any;
use std::any::type_name;
use std::ops::Deref;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub use autowired_derive::*;
pub use beans::Bean;
pub use inventory::submit;

mod beans;

fn component_mutex() -> &'static Mutex<u64> {
    static INSTANCE: OnceCell<Mutex<u64>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn component_dashmap() -> &'static DashMap<String, Arc<dyn Any + 'static + Send + Sync>> {
    static INSTANCE: OnceCell<DashMap<String, Arc<dyn Any + 'static + Send + Sync>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn bean_dashmap() -> &'static DashMap<String, Bean> {
    static INSTANCE: OnceCell<DashMap<String, Bean>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn get_component<T: Any + Send + Sync>() -> Option<Arc<T>> {
    component_dashmap().get(type_name::<T>())
        .map(|x| x.value().clone())
        .map(|x| x.downcast::<T>().ok())
        .flatten()
}

/// return true if component exists
pub fn exist_component<T>() -> bool {
    component_dashmap().contains_key(type_name::<T>())
}

pub trait Component: Any + 'static + Send + Sync {
    fn new_instance() -> Option<Self> where Self: Sized;
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

impl<T: Send + Sync + 'static> Deref for Autowired<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| {
            if !exist_component::<T>() {
                let name = type_name::<T>();
                if let Some(bean) = bean_dashmap().get(name) {
                    register_with_type_name(name.to_owned(), (bean.provider)());
                }
            }
            get_component::<T>().unwrap_or_else(||
                panic!("[Autowired] not found component {}", type_name::<T>())
            )
        })
    }
}

impl<T> Default for Autowired<T> {
    fn default() -> Self {
        Autowired::new()
    }
}

// fn init_and_register<T: Component>() -> bool {
//     register_with(|| T::new_instance().map(Into::<Arc<T>>::into))
// }

/// add component into a global map
/// return false if component has already existed or `constructor` return `None`
pub fn register_with<T: Component>(constructor: impl FnOnce() -> Option<Arc<T>>) -> bool {
    let name = type_name::<T>();
    if let Ok(mut count) = component_mutex().lock() {
        if component_dashmap().contains_key(name) {
            return false;
        }

        let component = match constructor() {
            None => return false,
            Some(c) => c,
        };
        component_dashmap().insert(name.to_string(), component.clone());
        *count += 1;

        log::debug!("[Component] register, name={}", name);
    }
    true
}

/// add component into a global map
/// return false if component has already existed
pub fn register<T: Component>(component: Arc<T>) -> bool {
    register_with(|| Some(component))
}

/// register with type name and instance
fn register_with_type_name(type_name: String, component: Arc<dyn Any + 'static + Send + Sync>) -> bool {
    let name = &type_name;
    if let Ok(mut count) = component_mutex().lock() {
        if component_dashmap().contains_key(name) {
            return false;
        }

        component_dashmap().insert(name.to_string(), component.clone());
        *count += 1;

        log::debug!("[Component] register, name={}", name);
    }
    true
}

/// register component which derives `Bean`
pub fn setup_submitted_beans() {
    for bean in inventory::iter::<Bean> {
        if !bean.lazy {
            register_with_type_name(bean.type_name.clone(), (bean.provider)());
        }
        bean_dashmap().insert(bean.type_name.clone(), bean.clone());
    }
}

/// get list of submitted bean name
pub fn list_bean_names() -> Vec<String> {
    bean_dashmap().iter().map(|x| x.type_name.clone()).collect()
}
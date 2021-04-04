use std::sync::Arc;
use std::any::Any;
use crate::Component;

#[derive(Clone)]
pub struct Bean {
    pub type_name: String,
    pub provider: Arc<dyn Fn() -> Arc<dyn Any + 'static + Send + Sync> + Send + Sync >,
    pub lazy: bool,
}

impl Bean {
    pub fn new_unchecked<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            provider: Arc::new(move || Arc::new(C::new_instance().unwrap())),
            lazy: false,
        }
    }

    pub fn from_fn<T: 'static + Send + Sync>(f: impl Fn() -> T + 'static + Send + Sync) -> Self {
        Self {
            type_name: std::any::type_name::<T>().to_string(),
            provider: Arc::new(move || Arc::new(f())),
            lazy: false,
        }
    }

    pub fn new_unchecked_lazy<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            provider: Arc::new(move || Arc::new(C::new_instance().unwrap())),
            lazy: true,
        }
    }

    pub fn from_fn_lazy<T: 'static + Send + Sync>(f: impl Fn() -> T + 'static + Send + Sync) -> Self {
        Self {
            type_name: std::any::type_name::<T>().to_string(),
            provider: Arc::new(move || Arc::new(f())),
            lazy: true,
        }
    }
}

inventory::collect!(Bean);

use std::sync::Arc;
use std::any::Any;
use crate::Component;

pub struct Bean {
    pub type_name: String,
    pub provider: Box<dyn Fn() -> Arc<dyn Any + 'static + Send + Sync>>,
}

impl Bean {
    pub fn new_unchecked<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            provider:Box::new( move || Arc::new(C::new_instance().unwrap())) ,
        }
    }

    pub fn from_fn<T: 'static + Send + Sync>(f: impl Fn() -> T + 'static) -> Self {
        Self {
            type_name: std::any::type_name::<T>().to_string(),
            provider: Box::new(move || Arc::new(f())),
        }
    }
}

inventory::collect!(Bean);
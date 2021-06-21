use std::any::Any;
use std::sync::Arc;

use crate::Component;

type DynAny = dyn Any + 'static + Send + Sync;

#[derive(Clone)]
pub struct Bean {
    pub type_name: String,
    pub provider: Arc<dyn Fn() -> Option<Arc<DynAny>> + Send + Sync>,
    pub lazy: bool,
}

impl Bean {
    pub fn new_unchecked<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            provider: Arc::new(move || C::new_instance().map(|x| Arc::new(x) as Arc::<DynAny>) ),
            lazy: false,
        }
    }

    pub fn from_fn<T: 'static + Send + Sync>(f: impl Fn() -> T + 'static + Send + Sync, lazy: bool) -> Self {
        Self {
            type_name: std::any::type_name::<T>().to_string(),
            provider: Arc::new(move || Some(f()).map(|x| Arc::new(x) as Arc::<DynAny>)),
            lazy,
        }
    }

    pub fn new_unchecked_lazy<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            provider: Arc::new(move || C::new_instance().map(|x| Arc::new(x) as Arc::<DynAny>)),
            lazy: true,
        }
    }

    pub fn from_fn_return_option<T>(f: impl Fn() -> Option<T> + 'static + Send + Sync, lazy: bool) -> Self
        where T: 'static + Send + Sync
    {
        Self {
            type_name: std::any::type_name::<T>().to_string(),
            provider: Arc::new(move || f().map(Arc::new).map(|x| x as Arc::<_>)),
            lazy,
        }
    }
}

inventory::collect!(Bean);

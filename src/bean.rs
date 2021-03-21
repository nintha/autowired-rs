use std::sync::Arc;
use crate::Component;
use std::any::Any;

pub struct Bean {
    pub type_name: String,
    pub component: Arc<dyn Any + 'static + Send + Sync>,
}

impl Bean {
    pub fn new_unchecked<C: Component>() -> Self {
        Self {
            type_name: std::any::type_name::<C>().to_string(),
            component: C::new_instance().ok().unwrap(),
        }
    }
}
inventory::collect!(Bean);
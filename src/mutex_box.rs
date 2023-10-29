use std::sync::Mutex;
use serde::{Deserialize,Serialize};

#[derive(Debug,  Default,Deserialize,Serialize)]
pub struct MutexBox<T>{
    pub mutex: Mutex<Option<T>>,
}

impl<T> MutexBox<T> 
{
    pub const fn new() -> Self {
        let mutex: Mutex<Option<T>> = Mutex::new(None);
        MutexBox {

            mutex: mutex,
        }
    }

    pub fn open_locked<FunctionLocked, TypeReturn>(
        &self,
        found: FunctionLocked,
        error_val: TypeReturn,
    ) -> TypeReturn
    where
        FunctionLocked: FnOnce(&mut T) -> TypeReturn,
    {
        if let Ok(mut handler_option) = self.mutex.lock() {
            if let Some(handler) = handler_option.as_mut() {
                found(handler)
            } else {
                error_val
            }
        } else {
            error_val
        }
    }

    pub fn init(&self, data: T) {
        self.mutex
            .lock()
            .expect(format!("could not lock ").as_str())
            .get_or_insert(data);
    }
}
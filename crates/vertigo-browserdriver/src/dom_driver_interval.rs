use std::{collections::HashMap, rc::Rc};

use vertigo::utils::{BoxRefCell, DropResource};
use wasm_bindgen::prelude::Closure;

use crate::DomDriverJsInterval;

struct CallbackManagerInner {
    id: u64,
    data: HashMap<u64, Rc<dyn Fn()>>,
}
impl CallbackManagerInner {
    pub fn set<F: Fn() + 'static>(&mut self, callback: F) -> u64 {
        let next_id = self.id;
        self.id += 1;
        self.data.insert(next_id, Rc::new(callback));
        next_id
    }

    pub fn get(&self, callback_id: u64) -> Option<Rc<dyn Fn()>> {
        if let Some(value) = self.data.get(&callback_id) {
            return Some(value.clone());
        }

        None
    }
}

#[derive(Clone)]
struct CallbackManager {
    state: Rc<BoxRefCell<CallbackManagerInner>>,
}

impl CallbackManager {
    pub fn new() -> CallbackManager {
        let state = CallbackManagerInner {
            id: 1,
            data: HashMap::new(),
        };

        CallbackManager {
            state: Rc::new(
                BoxRefCell::new(state, "CallbackManager")
            )
        }
    }

    pub fn set<F: Fn() + 'static>(&self, callback: F) -> u64 {
        self.state.change(callback, |state, callback| {
            state.set(callback)
        })
    }

    pub fn get(&self, callback_id: u64) -> Option<Rc<dyn Fn()>> {
        self.state.get_with_context(callback_id, |state, callback_id| {
            state.get(callback_id)
        }) 
    }
}

pub struct DomDriverInterval {
    driver_js: Rc<DomDriverJsInterval>,
    _closure: Closure<dyn Fn(u64)>,
    callback_manager: CallbackManager,
}

impl DomDriverInterval {
    pub fn new() -> DomDriverInterval {
        let callback_manager = CallbackManager::new();

        let closure = {
            let callback_manager = callback_manager.clone();

            Closure::new(Box::new(move |callback_id: u64| {
                log::info!("Interval ... {}", callback_id);

                let callback = callback_manager.get(callback_id);

                if let Some(callback) = callback {
                    callback();
                } else {
                    log::error!("Missing callback for id={}", callback_id);
                }
            }))
        };

        let driver_js = Rc::new(DomDriverJsInterval::new(&closure));

        DomDriverInterval {
            driver_js,
            _closure: closure,
            callback_manager
        }
    }

    pub fn set_interval<F: Fn() + 'static>(&self, time: u32, callback: F) -> DropResource {
        let callback_id = self.callback_manager.set(callback);

        let timer_id = self.driver_js.set_interval(time, callback_id);

        let driver_js = self.driver_js.clone();

        DropResource::new(move || {
            driver_js.clear_interval(timer_id);
        })
    }
}

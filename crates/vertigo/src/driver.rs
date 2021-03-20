use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    rc::Rc,
};

use crate::utils::{EqBox, DropResource};
use crate::virtualdom::models::realdom_id::RealDomId;

#[derive(Debug)]
pub enum FetchMethod {
    GET,
    POST,
}

impl FetchMethod {
    pub fn to_string(&self) -> &str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
        }
    }
}

pub enum FetchError {
    Error,
}

pub enum EventCallback {
    OnClick {
        callback: Option<Rc<dyn Fn()>>,
    },
    OnInput {
        callback: Option<Rc<dyn Fn(String)>>,
    },
                //mouseenter
    OnMouseEnter {
        callback: Option<Rc<dyn Fn()>>,
    },
                //mouseleave
    OnMouseLeave {
        callback: Option<Rc<dyn Fn()>>,
    },
}

impl EventCallback {
    pub fn to_string(&self) -> &str {
        match self {
            EventCallback::OnClick { callback} => {
                if callback.is_some() {
                    "onClick set"
                } else {
                    "onClick clear"
                }
            },
            EventCallback::OnInput { callback } =>{
                if callback.is_some() {
                    "onInput set"
                } else {
                    "onInput clear"
                }
            },
            EventCallback::OnMouseEnter { callback } =>{
                if callback.is_some() {
                    "onMouseEnter set"
                } else {
                    "onMouseEnter clear"
                }
            },
            EventCallback::OnMouseLeave { callback } =>{
                if callback.is_some() {
                    "onMouseLeave set"
                } else {
                    "onMouseLeave clear"
                }
            },
        }
    }
}

const SHOW_LOG: bool = true;

pub trait DomDriverTrait {
    fn create_node(&self, id: RealDomId, name: &'static str);
    fn create_text(&self, id: RealDomId, value: &str);
    fn update_text(&self, id: RealDomId, value: &str);
    fn set_attr(&self, id: RealDomId, key: &'static str, value: &str);
    fn remove_attr(&self, id: RealDomId, name: &'static str);
    fn remove(&self, id: RealDomId);
    fn insert_before(&self, parent: RealDomId, child: RealDomId, ref_id: Option<RealDomId>);
    fn insert_css(&self, selector: &str, value: &str);
    fn set_event(&self, node: RealDomId, callback: EventCallback);
    fn fetch(&self, method: FetchMethod, url: String, headers: Option<HashMap<String, String>>, body: Option<String>) -> Pin<Box<dyn Future<Output=Result<String, FetchError>> + 'static>>;

    fn get_hash_location(&self) -> String;
    fn push_hash_location(&self, path: &str);
    fn on_hash_route_change(&self, on_change: Box<dyn Fn(String)>) -> RoutingReceiver;
    fn clear_route_callback(&self);

    fn get_browser_location(&self) -> (String, String);
    fn push_browser_location(&self, path: &str);
    fn on_browser_route_change(&self, on_change: Box<dyn Fn((String, String))>) -> RoutingReceiver;

    fn set_interval(&self, time: u32, func: Box<dyn Fn()>) -> DropResource;
}

type Executor = Box<dyn Fn(Pin<Box<dyn Future<Output = ()> + 'static>>)>;

#[derive(PartialEq)]
pub struct DomDriver {
    driver: EqBox<Rc<dyn DomDriverTrait>>,
    spawn_local_executor: EqBox<Rc<Executor>>,
}

impl DomDriver {
    pub fn new<
        T: DomDriverTrait + 'static,
    >(driver: T, spawn_local: Executor) -> DomDriver {
        DomDriver {
            driver: EqBox::new(Rc::new(driver)),
            spawn_local_executor: EqBox::new(Rc::new(spawn_local))
        }
    }
}

impl Clone for DomDriver {
    fn clone(&self) -> DomDriver {
        DomDriver {
            driver: self.driver.clone(),
            spawn_local_executor: self.spawn_local_executor.clone(),
        }
    }
}

fn show_log(message: String) {
    if SHOW_LOG {
        log::info!("{}", message);
    }
}

impl DomDriver {
    pub fn spawn_local<F>(&self, future: F)
        where F: Future<Output = ()> + 'static {

            let fur = Box::pin(future);

            let spawn_local_executor = self.spawn_local_executor.clone();
            spawn_local_executor(fur)
}
    pub fn create_node(&self, id: RealDomId, name: &'static str) {
        show_log(format!("create_node {} {}", id, name));
        self.driver.create_node(id, name);
    }

    pub fn create_text(&self, id: RealDomId, value: &str) {
        show_log(format!("create_text {} {}", id, value));
        self.driver.create_text(id, value);
    }

    pub fn update_text(&self, id: RealDomId, value: &str) {
        show_log(format!("update_text {} {}", id, value));
        self.driver.update_text(id, value);
    }

    pub fn set_attr(&self, id: RealDomId, key: &'static str, value: &str) {
        show_log(format!("set_attr {} {} {}", id, key, value));
        self.driver.set_attr(id, key, value);
    }

    pub fn remove_attr(&self, id: RealDomId, name: &'static str) {
        show_log(format!("remove_attr {} {}", id, name));
        self.driver.remove_attr(id, name);
    }

    pub fn remove(&self, id: RealDomId) {
        show_log(format!("remove {}", id));
        self.driver.remove(id);
    }

    pub fn insert_before(&self, parent: RealDomId, child: RealDomId, ref_id: Option<RealDomId>) {
        match &ref_id {
            Some(ref_id) => {
                show_log(format!("insert_before child={} refId={}", child, ref_id));
            },
            None => {
                show_log(format!("insert_before child={} refId=None", child));
            }
        }

        self.driver.insert_before(parent, child, ref_id);
    }

    pub fn insert_css(&self, selector: &str, value: &str) {
        show_log(format!("insert_css selector={} value={}", selector, value));
        self.driver.insert_css(selector, value);
    }

    pub fn set_event(&self, node: RealDomId, callback: EventCallback) {
        show_log(format!("set_event {} {}", node, callback.to_string()));
        self.driver.set_event(node, callback);
    }

    pub fn fetch(&self, method: FetchMethod, url: String, headers: Option<HashMap<String, String>>, body: Option<String>) -> Pin<Box<dyn Future<Output=Result<String, FetchError>> + 'static>> {
        show_log(format!("fetch {:?} {}", method, url));
        self.driver.fetch(method, url, headers, body)
    }

    /* HashRouter methods */

    pub fn get_hash_location(&self) -> String {
        show_log("get_location".to_string());
        self.driver.get_hash_location()
    }

    pub fn push_hash_location(&self, path: String) {
        show_log(format!("set_location {}", path));
        self.driver.push_hash_location(&path)
    }

    pub fn on_hash_route_change(&self, on_change: Box<dyn Fn(String)>) -> RoutingReceiver {
        show_log("on_route_change".to_string());
        self.driver.on_hash_route_change(on_change)
    }

    pub fn clear_route_callback(&self) {
        self.driver.clear_route_callback()
    }

    /* BrowserRouter methods */

    pub fn get_browser_location(&self) -> (String, String) {
        let locat = self.driver.get_browser_location();
        show_log(format!("get_location: ({}, {})", locat.0, locat.1));
        locat
    }

    pub fn push_browser_location(&self, path: String) {
        show_log(format!("set_location {}", path));
        self.driver.push_browser_location(&path)
    }

    pub fn on_browser_route_change(&self, on_change: Box<dyn Fn((String, String))>) -> RoutingReceiver {
        show_log("on_route_change".to_string());
        self.driver.on_browser_route_change(on_change)
    }

    /* Interval */

    pub fn set_interval<F: Fn() + 'static>(&self, time: u32, func: F) -> DropResource{
        self.driver.set_interval(time, Box::new(func))
    }
}

#[derive(PartialEq)]
pub struct RoutingReceiver {
    driver: EqBox<Rc<dyn DomDriverTrait>>,
}

impl RoutingReceiver {
    pub fn new<T: DomDriverTrait + 'static>(driver: T) -> Self {
        Self {
            driver: EqBox::new(Rc::new(driver)),
        }
    }
}

impl Drop for RoutingReceiver {
    fn drop(&mut self) {
        self.driver.clear_route_callback()
    }
}

use std::rc::Rc;
use crate::vdom::models::{
    RealDomId::RealDomId,
};

const SHOW_LOG: bool = false;

pub trait DomDriverTrait {
    fn createNode(&self, id: RealDomId, name: &'static str);
    fn createText(&self, id: RealDomId, value: &str);
    fn createComment(&self, id: RealDomId, value: &str);
    fn setAttr(&self, id: RealDomId, key: &'static str, value: &str);
    fn removeAttr(&self, id: RealDomId, name: &'static str);
    fn remove(&self, id: RealDomId);
    fn insertAsFirstChild(&self, parent: RealDomId, child: RealDomId);
    fn insertBefore(&self, refId: RealDomId, child: RealDomId);
    fn insertAfter(&self, refId: RealDomId, child: RealDomId);
    fn addChild(&self, parent: RealDomId, child: RealDomId);

    fn insertCss(&self, selector: String, value: String);

    fn setOnClick(&self, node: RealDomId, onClick: Option<Rc<dyn Fn()>>);
}


pub struct DomDriver {
    driver: Rc<dyn DomDriverTrait>,
}

impl DomDriver {
    pub fn new<T: DomDriverTrait + 'static>(driver: T) -> DomDriver {
        DomDriver {
            driver: Rc::new(driver)
        }
    }
}

impl Clone for DomDriver {
    fn clone(&self) -> DomDriver {
        DomDriver {
            driver: self.driver.clone()
        }
    }
}

fn show_log(message: String) {
    if SHOW_LOG {
        log::info!("{}", message);
    }
}
impl DomDriver {
    pub fn createNode(&self, id: RealDomId, name: &'static str) {
        show_log(format!("createNode {} {}", id, name));
        self.driver.createNode(id, name);
    }

    pub fn createText(&self, id: RealDomId, value: &str) {
        show_log(format!("createText {} {}", id, value));
        self.driver.createText(id, value);
    }

    pub fn createComment(&self, id: RealDomId, value: &str) {
        show_log(format!("createComment {} {}", id, value));
        self.driver.createComment(id, value);
    }

    pub fn setAttr(&self, id: RealDomId, key: &'static str, value: &str) {
        show_log(format!("setAttr {} {}", key, value));
        self.driver.setAttr(id, key, value);
    }

    pub fn removeAttr(&self, id: RealDomId, name: &'static str) {
        show_log(format!("removeAttr {} {}", id, name));
        self.driver.removeAttr(id, name);
    }

    pub fn remove(&self, id: RealDomId) {
        show_log(format!("remove {}", id));
        self.driver.remove(id);
    }

    pub fn insertAsFirstChild(&self, parent: RealDomId, child: RealDomId) {
        show_log(format!("insertAsFirstChild parent={} child={}", parent, child));
        self.driver.insertAsFirstChild(parent, child);
    }

    pub fn insertBefore(&self, refId: RealDomId, child: RealDomId) {
        show_log(format!("insertBefore refId={} child={}", refId, child));
        self.driver.insertBefore(refId, child);
    }

    pub fn insertAfter(&self, refId: RealDomId, child: RealDomId) {
        show_log(format!("insertAfter refId={} child={}", refId, child));
        self.driver.insertAfter(refId, child);
    }

    pub fn addChild(&self, parent: RealDomId, child: RealDomId) {
        show_log(format!("addChild parent={} child={}", parent, child));
        self.driver.addChild(parent, child);
    }

    pub fn insertCss(&self, selector: String, value: String) {
        show_log(format!("insertCss selector={} value={}", selector, value));
        self.driver.insertCss(selector, value);
    }

    pub fn setOnClick(&self, node: RealDomId, onClick: Option<Rc<dyn Fn()>>) {
        show_log(format!("setOnClick {} --onClick--", node));
        self.driver.setOnClick(node, onClick);
    }
}

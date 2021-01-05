use std::collections::{
    HashMap,
    VecDeque,
};
use std::rc::Rc;
use crate::{driver::{DomDriver, EventCallback}, virtualdom::{
        models::{
            realdom::RealDom,
            realdom_id::RealDomId,
            realdom_text::RealDomText,
        },
    }};
use crate::utils::BoxRefCell;


fn merge_attr(attr: &HashMap<&'static str, String>, class_name: Option<String>) -> HashMap<&'static str, String> {
    let mut attr = attr.clone();

    if let Some(class_name) = class_name {
        let attr_class = attr.get("class");

        let value_to_set: String = match attr_class {
            Some(attr_class) => format!("{} {}", class_name, attr_class),
            None => class_name
        };

        attr.insert("class", value_to_set);
    }

    attr
}

pub struct RealDomNodeInner {
    dom_driver: DomDriver,
    pub id_dom: RealDomId,
    pub name: &'static str,
    attr: HashMap<&'static str, String>,
    pub child: VecDeque<RealDom>,
}

impl RealDomNodeInner {
    pub fn new(driver: DomDriver, name: &'static str) -> RealDomNodeInner {
        let node_id = RealDomId::default();

        driver.create_node(node_id.clone(), name);

        RealDomNodeInner {
            dom_driver: driver,
            id_dom: node_id,
            name,
            attr: HashMap::new(),
            child: VecDeque::new(),
        }
    }

    pub fn create_with_id(driver: DomDriver, id: RealDomId) -> RealDomNodeInner {
        RealDomNodeInner {
            dom_driver: driver,
            id_dom: id,
            name: "div",
            attr: HashMap::new(),
            child: VecDeque::new(),
        }
    }

    fn update_attr_one(&mut self, name: &'static str, value: &str) {
        let need_update = {
            let item = self.attr.get(name);
            if let Some(item) = item {
                *item != *value
            } else {
                true
            }
        };

        if need_update {
            self.dom_driver.set_attr(self.id_dom.clone(), &name, &value);
            self.attr.insert(name, value.to_string());
       }
    }

    pub fn update_attr(&mut self, attr: &HashMap<&'static str, String>, class_name: Option<String>) {
        let attr = merge_attr(attr, class_name);

        let mut to_delate: Vec<&str> = Vec::new();

        for (key, _) in self.attr.iter() {
            if !attr.contains_key(*key) {
                to_delate.push(*key);
            }
        }

        for key_to_delete in to_delate.into_iter() {
            self.dom_driver.remove_attr(self.id_dom.clone(), key_to_delete)
        }

        self.attr.retain(|key, _value| {
            let key: &str = *key;

            attr.contains_key(key)
        });

        for (key, value) in attr.iter() {
            self.update_attr_one(key, value);
        }
    }

    pub fn set_event(&mut self, callback: EventCallback) {
        self.dom_driver.set_event(self.id_dom.clone(), callback);
    }

    pub fn extract_child(&mut self) -> VecDeque<RealDom> {
        std::mem::replace(&mut self.child, VecDeque::new())
    }

    pub fn put_child(&mut self, child: VecDeque<RealDom>) -> VecDeque<RealDom> {
        std::mem::replace(&mut self.child, child)
    }

    pub fn insert_before(&mut self, new_child: RealDom, prev_node: Option<RealDomId>) {
        self.dom_driver.insert_before(self.id_dom.clone(), new_child.id(), prev_node);
        self.child.push_front(new_child);
    }
}

impl Drop for RealDomNodeInner {
    fn drop(&mut self) {
        self.dom_driver.remove(self.id_dom.clone());
    }
}


pub struct RealDomNode {
    inner: Rc<BoxRefCell<RealDomNodeInner>>,
}

impl RealDomNode {
    pub fn new(driver: DomDriver, name: &'static str) -> RealDomNode {
        RealDomNode {
            inner: Rc::new(
                BoxRefCell::new(
                    RealDomNodeInner::new(driver, name)
                )
            )
        }
    }

    pub fn create_with_id(driver: DomDriver, id: RealDomId) -> RealDomNode {
        RealDomNode {
            inner: Rc::new(
                BoxRefCell::new(
                    RealDomNodeInner::create_with_id(driver, id)
                )
            )
        }
    }

    pub fn update_attr(&self, attr: &HashMap<&'static str, String>, class_name: Option<String>) {
        self.inner.change(
            (attr, class_name),
            |state, (attr, class_name)| {
                state.update_attr(attr, class_name)
        })
    }

    pub fn set_event(&self, callback: EventCallback) {
        self.inner.change(
            callback,
            |state, callback| {
                state.set_event(callback)
        })
    }

    pub fn id_dom(&self) -> RealDomId {
        self.inner.get(
            |state| {
                state.id_dom.clone()
        })
    }

    pub fn name(&self) -> &'static str {
        self.inner.get(
            |state| {
                state.name
        })
    }

    pub fn extract_child(&self) -> VecDeque<RealDom> {
        self.inner.change(
            (),
            |state, ()| {
                state.extract_child()
        })
    }

    pub fn put_child(&self, child: VecDeque<RealDom>) {
        self.inner.change(child, |state, child| {
            state.put_child(child);
        })
    }

    pub fn insert_before(&self, new_child: RealDom, prev_node: Option<RealDomId>) {
        self.inner.change(
            (new_child, prev_node),
            |state, (new_child, prev_node)| {
                state.insert_before(new_child, prev_node)
        })
    }

    fn dom_driver(&self) -> DomDriver {
        self.inner.get(
            |state| {
                state.dom_driver.clone()
        })
    }

    pub fn create_node(&self, name: &'static str) -> RealDomNode {
        RealDomNode::new(self.dom_driver(), name)
    }

    pub fn create_text(&self, name: String) -> RealDomText {
        RealDomText::new(self.dom_driver(), name)
    }
}


impl Clone for RealDomNode {
    fn clone(&self) -> Self {
        RealDomNode {
            inner: self.inner.clone()
        }
    }
}
use std::rc::Rc;

use crate::vdom::models::{
    VDom::VDom,
    VDomNode::VDomNode,
    VDomComponent::VDomComponent,
};
use crate::vdom::models::Css::Css;
use crate::computed::{
    Computed::Computed,
};

pub enum NodeAttr {
    Css {
        css: Css
    },
    OnClick {
        event: Rc<dyn Fn()>
    },
    Attr {
        name: &'static str,
        value: String,
    },
    Node {
        node: VDom,
    }
}



pub fn css(css: Css) -> NodeAttr {
    NodeAttr::Css {
        css,
    }
}

pub fn onClick<F: Fn() + 'static>(callback: F) -> NodeAttr {
    NodeAttr::OnClick {
        event: Rc::new(callback),
    }
}

pub fn attr<K: Into<String>>(name: &'static str, value: K) -> NodeAttr {
    NodeAttr::Attr {
        name,
        value: value.into()
    }
}

pub fn node(name: &'static str, childList: Vec<NodeAttr>) -> NodeAttr {
    NodeAttr::Node {
        node: VDom::node(name, childList)
    }
}

pub fn text<T: Into<String>>(name: T) -> NodeAttr {
    NodeAttr::Node {
        node: VDom::text(name)
    }
}

pub fn component<T: 'static>(params: Computed<T>, render: fn(Rc<T>) -> VDomNode) -> NodeAttr {
    NodeAttr::Node {
        node:VDom::component(VDomComponent::new(params, render))
    }
}

pub fn buildNode(name: &'static str, childList: Vec<NodeAttr>) -> VDom {
    VDom::node(name, childList)
}

pub fn buildText<T: Into<String>>(name: T) -> VDom {
    VDom::text(name)
}

use std::collections::{
    HashMap,
    VecDeque,
};
use std::hash::Hash;

use crate::{
    computed::{
        Client,
    },
    driver::EventCallback
};

use crate::{
    virtualdom::{
        models::{
            realdom::RealDom,
            realdom_node::RealDomNode,
            realdom_text::RealDomText,
            realdom_component::RealDomComponent,
            vdom::VDom,
            vdom_node::VDomNode,
            vdom_component::VDomComponent,
            vdom_component_id::VDomComponentId,
            vdom_text::VDomText,
            realdom_id::RealDomId,
        }
    },
    css_manager::css_manager::CssManager,
};

struct CacheNode<K: Eq + Hash, RNode, VNode> {
    create_new: fn(&CssManager, &RealDomNode, &VNode) -> RNode,
    data: HashMap<K, VecDeque<RNode>>,
}

impl<K: Eq + Hash, RNode, VNode> CacheNode<K, RNode, VNode> {
    fn new(
        create_new: fn(&CssManager, &RealDomNode, &VNode) -> RNode,
    ) -> CacheNode<K, RNode, VNode> {
        CacheNode {
            create_new,
            data: HashMap::new()
        }
    }

    fn insert(&mut self, key: K, node: RNode) {
        let item = self.data.entry(key).or_insert_with(VecDeque::new);
        item.push_back(node);
    }

    fn get_or_create(&mut self, css_manager: &CssManager, target: &RealDomNode, key: K, vnode: &VNode) -> RNode {
        let item = self.data.entry(key).or_insert_with(VecDeque::new);

        let node = item.pop_front();

        let CacheNode { create_new, .. } = self;

        match node {
            Some(node) => node,
            None => create_new(css_manager, target, &vnode)
        }
    }
}

enum NodePairs<'a> {
    Component {
        real: &'a RealDomComponent,
        new: &'a VDomComponent
    },
    Node {
        real: &'a RealDomNode,
        new: &'a VDomNode,
    },
    Text {
        real: &'a RealDomText,
        new: &'a VDomText,
    }
}

fn get_pair_for_update<'a>(real: &'a RealDom, new: &'a VDom) -> Option<NodePairs<'a>> {
    match real {
        RealDom::Component { node } => {
            if let VDom::Component { node: vnode } = new {
                if node.id == vnode.id {
                    return Some(NodePairs::Component {
                        real: node,
                        new: vnode
                    });
                }
            }
        },
        RealDom::Node { node } => {
            if let VDom::Node { node : vnode} = new {
                if node.name() == vnode.name {
                    return Some(NodePairs::Node {
                        real: node,
                        new: vnode,
                    });
                }
            }
        },
        RealDom::Text { node } => {
            if let VDom::Text { node: vnode } = new {
                return Some(NodePairs::Text {
                    real: node,
                    new: vnode
                });
            }
        }
    }

    None
}

fn update_node_child_updated_with_order(css_manager: &CssManager, target: &VecDeque<RealDom>, new_version: &Vec<VDom>) -> bool {
    if target.len() != new_version.len() {
        return false;
    }

    let max_index = target.len();

    let mut for_update: Vec<NodePairs> = Vec::new();

    for index in 0..max_index {
        let real = &target[index];
        let new = &new_version[index];

        if let Some(pair) = get_pair_for_update(real, new) {
            for_update.push(pair);
        } else {
            return false;
        }
    }

    for item in for_update {
        match item {
            NodePairs::Component { real: _real, new: _new } => {
            },
            NodePairs::Node { real, new } => {
                update_node_attr(&css_manager, real, new);
                update_node_child(css_manager, real, new);
            },
            NodePairs::Text { real, new } => {
                real.update(&new.value);
            },
        }
    }

    true
}

fn update_node_child(css_manager: &CssManager, target: &RealDomNode, new_version: &VDomNode) {

    let mut real_child = target.extract_child();

    let update_order_ok = update_node_child_updated_with_order(css_manager, &mut real_child, &new_version.child);
    if update_order_ok {
        target.put_child(real_child);
        return;
    }

    let mut real_node: CacheNode<&'static str, RealDomNode, VDomNode> = CacheNode::new(
        |_css_manager: &CssManager, target: &RealDomNode, node: &VDomNode| -> RealDomNode {
            target.create_node(node.name)
        },
    );
    let mut real_text: CacheNode<String, RealDomText, VDomText> = CacheNode::new(
        |_css_manager: &CssManager, target: &RealDomNode, node: &VDomText| -> RealDomText {
            target.create_text(node.value.clone())
        },
    );
    let mut real_component: CacheNode<VDomComponentId, RealDomComponent, VDomComponent> = CacheNode::new(
        |css_manager: &CssManager, target: &RealDomNode, node: &VDomComponent| -> RealDomComponent {

            let node_root = target.create_node("div");

            let node_root_for_id = node_root.clone();

            let subscription = render_to_node(css_manager.clone(), node_root, node.clone());

            RealDomComponent {
                id: node.id.clone(),
                subscription,
                node: node_root_for_id,
            }
        },
    );

    for item in real_child {
        match item {
            RealDom::Node { node }=> {
                real_node.insert(node.name(), node);
            },
            RealDom::Text { node } => {
                let id = node.get_value();
                real_text.insert(id, node);
            },
            RealDom::Component { node } => {
                let id = node.id.clone();
                real_component.insert(id, node);
            }
        }
    }


    let mut wsk: Option<RealDomId> = None;

    for item in new_version.child.iter().rev() {

        match item {
            VDom::Node { node } => {
                let id = node.name;
                let dom_child = real_node.get_or_create(css_manager, target, id, node);
                let new_wsk = dom_child.id_dom();

                update_node_attr(&css_manager, &dom_child, &node);
                update_node_child(css_manager, &dom_child, &node);

                target.insert_before(RealDom::Node { node: dom_child }, wsk);
                wsk = Some(new_wsk);
            },
            VDom::Text { node } => {
                let id = node.value.clone();
                let dom_child = real_text.get_or_create(css_manager, target,id, node);
                let new_wsk = dom_child.id_dom.clone();

                dom_child.update(&node.value);

                target.insert_before(RealDom::Text { node: dom_child }, wsk);
                wsk = Some(new_wsk);
            },
            VDom::Component { node } => {
                let id = node.id.clone();
                let dom_child = real_component.get_or_create(css_manager, target,id, node);
                let new_wsk = dom_child.dom_id();

                target.insert_before(RealDom::Component { node: dom_child }, wsk);
                wsk = Some(new_wsk);
            }
        }
    }
}


fn update_node_attr(css_manager: &CssManager, real_node: &RealDomNode, node: &VDomNode) {
    let css = &node.css;
    let class_name = match css {
        Some (css) => Some(css_manager.get_class_name(css)),
        None => None,
    };

    real_node.update_attr(&node.attr, class_name);
    real_node.set_event(EventCallback::OnClick { callback: node.on_click.clone() });
    real_node.set_event(EventCallback::OnInput { callback: node.on_input.clone() });
}

fn update_node(css_manager: &CssManager, target: &RealDomNode, new_version: &VDomNode) {

    //updejt nazwy taga ...
    //TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    //updejt atrybutów
    update_node_attr(&css_manager, target, &new_version);

    //odpal updejt dzieci
    update_node_child(css_manager, target, &new_version);
}

pub fn render_to_node(css_manager: CssManager, target: RealDomNode, component: VDomComponent) -> Client {
    let subscription: Client = component.view.subscribe(move |new_version| {
        update_node(
            &css_manager,
            &target,
            new_version
        );
    });

    subscription
}
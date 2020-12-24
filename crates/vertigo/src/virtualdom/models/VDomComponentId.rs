use crate::computed::{
    Computed,
    GraphId,
};

use crate::{
    virtualdom::{
        models::VDomNode::VDomNode,
    }
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VDomComponentId {
    idComputed: GraphId,        //id tego computed
    idFunction: u64,            //id tej konkretnej funkcji statycznej (renderującej komponent)
}

impl VDomComponentId {
    pub fn new<T>(params: &Computed<T>, render: fn(&Computed<T>) -> VDomNode) -> VDomComponentId {

        let idFunction = render as *const () as u64;
        VDomComponentId {
            idComputed: params.get_id(),
            idFunction
        }
    }
}

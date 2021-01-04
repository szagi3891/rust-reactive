use crate::computed::{
    Value,
    Computed,
    GraphId,
};

use crate::{
    virtualdom::{
        models::v_dom_node::VDomNode,
    }
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VDomComponentId {
    id_computed: GraphId,        //id tego computed
    id_function: u64,            //id tej konkretnej funkcji statycznej (renderującej komponent)
}

impl VDomComponentId {
    pub fn new<T: PartialEq>(params: &Computed<T>, render: fn(&Computed<T>) -> VDomNode) -> VDomComponentId {
        let id_function = render as *const () as u64;
        VDomComponentId {
            id_computed: params.get_id(),
            id_function
        }
    }

    pub fn new_value<T: PartialEq>(params: &Value<T>, render: fn(&Value<T>) -> VDomNode) -> VDomComponentId {
        let id_function = render as *const () as u64;
        VDomComponentId {
            id_computed: params.id(),
            id_function
        }
    }
}

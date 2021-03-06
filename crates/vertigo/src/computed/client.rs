use std::rc::Rc;

use crate::computed::{
    Dependencies,
    Computed,
    GraphValue,
    GraphId,
};
use crate::utils::EqBox;

#[derive(PartialEq)]
pub struct Client {
    graph_value: EqBox<GraphValue<()>>,
}

impl Client {
    pub fn new<T: PartialEq + 'static, F: Fn(&T) + 'static>(deps: Dependencies, computed: Computed<T>, call: F) -> Client {
        let graph_value = GraphValue::new_client(&deps, move || {
            let value = computed.get_value();
            call(value.as_ref());
            Rc::new(())
        });

        let _ = graph_value.get_value(false);

        Client {
            graph_value: EqBox::new(graph_value)
        }
    }

    pub fn off(self: Client) {
    }

    pub fn id(&self) -> GraphId {
        self.graph_value.id()
    }
}

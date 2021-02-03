use std::cmp::PartialEq;
use vertigo::{
    node_attr,
    VDomElement,
    computed::{
        Computed,
        Dependencies
    }
};

use vertigo_html::{Inline, html_component};

mod simple_counter;

#[derive(PartialEq)]
pub struct State {
    pub counter1: Computed<simple_counter::State>,
    pub counter2: Computed<simple_counter::State>,
    pub counter3: Computed<simple_counter::State>,
    pub counter4: Computed<simple_counter::State>,

    pub suma: Computed<u32>,
}

impl State {
    pub fn new(root: &Dependencies) -> Computed<State> {
        let counter1 = simple_counter::State::new(&root);
        let counter2 = simple_counter::State::new(&root);
        let counter3 = simple_counter::State::new(&root);
        let counter4 = simple_counter::State::new(&root);

        let suma = {
            let counter1 = counter1.clone();
            let counter2 = counter2.clone();
            let counter3 = counter3.clone();
            let counter4 = counter4.clone();

            root.from(move || {
                let value1 = *counter1.get_value().counter.get_value();
                let value2 = *counter2.get_value().counter.get_value();
                let value3 = *counter3.get_value().counter.get_value();
                let value4 = *counter4.get_value().counter.get_value();

                value1 + value2 + value3 + value4
            })
        };

        root.new_computed_from(State {
            counter1,
            counter2,
            counter3,
            counter4,
            suma,
        })
    }
}

fn render_suma(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();

    let suma = state.suma.get_value();

    html_component!(r#"
        <div>
            {$ format!("suma = {}", suma) $}
        </div>
    "#)
}

pub fn render(state: &Computed<State>) -> VDomElement {
    use node_attr::component;

    let suma = component(state.clone(), render_suma);

    let state = state.get_value();

    html_component!("
        <div>
            <component {simple_counter::render} data={state.counter1.clone()} />
            <component {simple_counter::render} data={state.counter2.clone()} />
            <component {simple_counter::render} data={state.counter3.clone()} />
            <component {simple_counter::render} data={state.counter4.clone()} />
            { suma }
        </div>
    ")
}

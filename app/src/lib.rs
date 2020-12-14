#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod app;
mod simple_counter;
mod fetch;
mod sudoku;

use wasm_bindgen::prelude::*;


use std::cell::RefCell;

use virtualdom::{
    computed::{
        Dependencies::Dependencies,
    },
    vdom::{
        App::App,
    },
    vdom::models::{
        VDomComponent::VDomComponent,
    },
};

use browserdriver::{
    DomDriverBrowser,
};

use crate::app::app_state::AppState;
use crate::app::app_render::main_render;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static APP_STATE: RefCell<App> = RefCell::new({
        let root: Dependencies = Dependencies::default();
        let appStateBox = AppState::new(&root);

        let driver = DomDriverBrowser::default();

        App::new(driver, VDomComponent::new(appStateBox, main_render))
    });
}

#[wasm_bindgen(start)]
pub async fn start_app() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Start rustowego modułu ...");

    APP_STATE.with(|state| state.borrow().start_app());

    wasm_bindgen_futures::spawn_local(async {
        log::info!("test z forka");
    });

    let aa = fetch::run("rustwasm/wasm-bindgen".into()).await;  //.unwrap();

    match aa {
        Ok(branch) => {
            log::info!("odpowiedź z serwera {:?}", branch);

        },
        Err(err) => {
            log::info!("błąd pobierania danych {:?}", err);
        }
    }
}


/*
TODO - wydzielić computed do osobnego crates


TODO - obadac ten sposób odpalania projektu wasm
    https://github.com/IMI-eRnD-Be/wasm-run


TODO - Dodać tranzakcyjną aktualizację
    self.deps.triggerChange([id1, id2, id3]);               //to powinno wystarczyc

TODO - Graph - usunac nieuzywane krawedzie (subskrybcje)

TODO - Graph - zamienić Clone na Copy

TODO - dodać jakieś makra które pozwolą na łatwe generowanie html-a (https://docs.rs/maplit/1.0.2/maplit/)
    to wygląda obiecująco
    https://github.com/chinedufn/percy/tree/master/crates/html-macro

TODO - Będąc w bloku computed, albo subskrybcji, całkowicie ignorować wszelkie akcje które będą chciały zmienić wartość
       rzucać standardowy strumień błędów informację o incydencie. Dzięki temu nowa wadliwa funkcjonalność nie zepsuje tej juz dobrze ulezanej funkcjonalności

https://github.com/rustwasm/console_error_panic_hook#readme
https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html


TODO - insertAsFirstChild, insertAfter. Wywalić te dwie funkcje.
    trzeba odwrócić kolejność synchronizowania węzłów. Korzystać z metody insertBefore. Najmniejszy narzut pod kątem ilości zmian w domie



TODO - makro które wycina białe znaki ?
Css::one("
        margin: 5px;
    ")


TODO - zrobić analizator Cargo.lock, wyszukiwać biblioteki w rónych wersjach które posiadają zmienne globalne
    przykład tokio ....
*/




/*
#[wasm_bindgen(start)]
pub fn main() {
    future_to_promise(
         Request::new(Method::Get, "example.org/test")
            .header("Accept", "text/plain").send()
            .and_then(|resp_value: JsValue| {
                let resp: Response = resp_value.dyn_into().unwrap();
                resp.text()
            })
            .and_then(|text: Promise| {
                JsFuture::from(text)
            })
            .and_then(|body| {
                println!("Response: {}", body.as_string().unwrap());
                future::ok(JsValue::UNDEFINED)
            })
    );
}
*/
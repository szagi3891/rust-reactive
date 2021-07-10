use vertigo::{DomDriver, VDomElement, computed::{
        Value,
        Computed,
        Dependencies
    }, utils::DropResource};

use vertigo_html::{html, css_fn, css_fn_push};

mod spinner;

use spinner::spinner;

#[derive(PartialEq)]
pub struct Animacja {
    pub value: Value<u32>,
    licznik_punktow: Value<u32>,
    _timer: DropResource,
}

impl Animacja {
    pub fn new(root: &Dependencies, driver: &DomDriver, licznik_punktow: Value<u32>) -> Computed<Animacja> {
        let value = root.new_value(0);

        let timer = {
            let value = value.clone();
            driver.set_interval(1000, move || {
                let val = value.get_value();
                value.set_value(*val + 1);
                log::info!("tik");
            })
        };

        root.new_computed_from(Animacja { 
            value,
            licznik_punktow,
            _timer: timer,
        })
    }

    pub fn ustaw(&self, new_value: u32) {
        let nowy_punkty = self.value.get_value();
        let punkty = self.licznik_punktow.get_value();
        self.licznik_punktow.set_value(*nowy_punkty + *punkty);

        self.value.set_value(new_value);
    }
}

fn animacja_rysuj(animacja: &Computed<Animacja>) -> VDomElement {
    
    let curr = animacja.get_value().value.get_value();
    let curr = if *curr < 50 { *curr } else { 50 };

    let mut out = Vec::new();

    for i in 0..(curr + 1) {
        out.push(format!("{} ", i));
    }

    let out_str = out.as_slice().join("");

    let wyzeruj = {
        let animacja = animacja.get_value();
        move || {
            animacja.ustaw(0);
        }
    };

    html! {
        <div>
            {out_str}
            <button onClick={wyzeruj}>"wyzeruj"</button>
        </div>
    }
}

#[derive(PartialEq)]
pub struct MainState {
    driver: DomDriver,
    pub value: Value<u32>,
    pub licznik_punktow: Value<u32>,
    pub animacje: Vec<Computed<Animacja>>,
}

impl MainState {
    pub fn new(root: &Dependencies, driver: DomDriver) -> Computed<MainState> {

        let licznik_punktow = root.new_value(0);
        let mut animacje = Vec::new();

        for _ in 0..21 {
            animacje.push(Animacja::new(root, &driver, licznik_punktow.clone()));
        }

        root.new_computed_from(MainState {
            value: root.new_value(30),
            driver,
            licznik_punktow,
            animacje,
        })
    }

    pub fn increment(&self) {
        let rr = self.value.get_value();
        self.value.set_value(*rr + 1);
    }

    pub fn decrement(&self) {
        let rr = self.value.get_value();
        self.value.set_value(*rr - 1);
    }
}

css_fn! { css_bg, "
    border: 1px solid black;
    padding: 10px;
    background-color: #e0e0e0;
    margin-bottom: 10px;
" }

css_fn_push! { css_button, css_bg, "
    cursor: pointer;
" }

pub fn main_render(state: &Computed<MainState>) -> VDomElement {
    let state = state.get_value();
    let value = *state.value.get_value();

    let on_down = {
        let app_state = state.clone();
        move || {
            app_state.decrement();
        }
    };

    let on_up = {
        let app_state = state.clone();
        move || {
            log::info!("on click");
            app_state.increment();
        }
    };

    let wyzeruj = {
        let app_state = state.clone();
        move || {
            app_state.value.set_value(0);
        }
    };

    let ustaw_sto = {
        let app_state = state.clone();
        move || {
            app_state.value.set_value(100);
        }
    };

    let podwoj_liczbe = {
        let app_state = state.clone();
        move || {
            let value = app_state.value.get_value();
            app_state.value.set_value(*value * 2);
        }
    };

    let pomniejsz_liczbe = {
        let app_state = state.clone();
        move || {
            let value = app_state.value.get_value();
            app_state.value.set_value(*value / 2);
        }
    };

    let wynik = if value > 35 {
        format!("Weronika {}", value * 20)
    } else {
        format!("Internet ==> {}", 4 * value)
    };

    let value2 = value * 2;
    let value_polowka = value / 2;

    let mut animacje_html = Vec::new();

    for animacja in state.animacje.iter() {
        animacje_html.push(html!{
            <component {animacja_rysuj} data={animacja.clone()} />
        });
    }

    let licznik_punktow = state.licznik_punktow.get_value();
    let punkty_string = format!("punktów = {}", *licznik_punktow);

    html! {
        <div aaa="one" bbb="two">
            <div>
                <div>
                    { punkty_string.clone() }
                </div>
                <div>
                { ..animacje_html }
                </div>
            </div>

            <div>"Abudabi"</div>
            <div css={css_bg()}>
                { wynik }
                { spinner() }
            </div>
            <div css={css_bg()} onClick={pomniejsz_liczbe}>
                "Actual value = " { value } " Jak klikniesz, to zmniejszę dwa razy --> " { value_polowka }
            </div>
            <div css={css_bg()} onClick={podwoj_liczbe}>
                "Actual value: " { value } " Jak klikniesz, to podwoję tą wartość --> " {value2}
            </div>
            <div css={css_button()} onClick={on_up}>
                "Zwiększ liczbę"
            </div>
            <div css={css_button()} onClick={on_down}>
                "Zmiejsz liczbę"
            </div>
            <button onClick={wyzeruj}>"Wyzeruj"</button>
            <button onClick={ustaw_sto}>"Ustaw stoo"</button>

            { punkty_string }
        </div>
    }
}
//            
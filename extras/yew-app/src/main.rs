use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    // || starts new closure, Model returned
    let state = use_state(|| Model {
        value: 0
    });

    // new state var, because original used by yew
    let onclick = {
        let state = state.clone(); // shadow

        // Callback expects a closure
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        }) // This is returned to onclick
    };

    html! {
        <div>
            <button {onclick}>
                { "+1" }
            </button>
            <p>{ state.value }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
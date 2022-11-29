# Rust Playground

```sh
cargo install trunk # build tool

# This allows building for WA
rustup target add wasm32-unknown-unknown

# Initialize new cargo project.
cargo init # or cargo new FolderName

# Build and serve project
trunk serve --open

```

Add `yew` dependency in toml file
```toml
[dependencies]
yew = "^0.19"
```

Change `main.rs` to say,
```rs
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
        let state = state.clone() // shadow

        // Callback expects a closure
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    }
}

html! {
    <div>
        <buton onclick={onclick}>{"+1"}</button>
        <p> { state.value} </p>
    </div>
}

fn main() {
    yew:start_app::<App>();
}
```

## Neglected reading
If you neglected reading the above, you will receive the following,
```
$ .\target\debug\yew-app.exe RUST_BACKTRACE=1
thread 'main' panicked at 'cannot call wasm-bindgen imported functions on non-wasm targets', C:\Users\Admin\.cargo\registry\src\github.com-1ecc6299db9ec823\js-sys-0.3.60\src\lib.rs:5520:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
fn App() -> Html {
    let running = use_state(|| false);
    let t0 = use_state(|| chrono::Local::now().timestamp_millis());
    let time = use_state(|| 0.0f32);

    use_interval(
        {
            let t0 = t0.clone();
            let time = time.clone();
            let running = running.clone();
            move || {
                if *running {
                    let duration = chrono::Local::now().timestamp_millis() - *t0;
                    let df32 = (duration as f32) / 1000f32;
                    time.set(df32);
                }
            }
        },
        10,
    );

    let start = {
        let running = running.clone();
        let t0 = t0.clone();
        let time = time.clone();
        move |_| {
            if *running {
                running.set(false);
                let duration = chrono::Local::now().timestamp_millis() - *t0;
                let df32 = (duration as f32) / 1000f32;
                time.set(df32);
            } else {
                t0.set(chrono::Local::now().timestamp_millis());
                running.set(true);
            }
        }
    };

    html! {
        <div>
            <button onclick={start}>{"Start"}</button>
            <p>{ "Hello world" }</p>
            <p>{ *time }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

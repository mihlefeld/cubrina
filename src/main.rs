use yew::prelude::*;
use yew_hooks::prelude::*;
mod timer;
use timer::DisplayTime; 
use timer::TimerHistory;
use timer::Timer;

#[function_component]
fn App() -> Html {
    let history = use_mut_ref(|| TimerHistory::new());
    let last_time = use_state(|| DisplayTime::new());

    // use_interval(
    //     {
    //     },
    //     10,
    // );
    let new_time_callback = Callback::<DisplayTime>::from(
        {
            let history = history.clone();
            let last_time = last_time.clone();
            move |time| {
                last_time.set(time);
                history.borrow_mut().push(time);
            }
        }
    );


    html! {
        <div>
            <p>{ "Hello world" }</p>
            <Timer finished={new_time_callback}></Timer>
            <p>{last_time.display()}</p>
            <ul>
                {
                    history.borrow().time_list.iter().map(|time| html! { <li>{ time.display() }</li> }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use yew_hooks::prelude::*;

use super::displaytime::DisplayTime;

#[derive(PartialEq, Properties)]
pub struct TimerProps {
    pub finished: Callback<DisplayTime>
}

#[function_component]
pub fn Timer(props: &TimerProps) -> Html {
    let done_callback = props.finished.clone();
    let timer_time = use_state(|| DisplayTime::new());
    let start_time = use_state(|| None::<i64>);
    {
        let timer_time = timer_time.clone();
        let start_time = start_time.clone();
        use_interval(
            move || {
                match *start_time {
                    Some(t) => timer_time.set(DisplayTime { millis: chrono::Utc::now().timestamp_millis() - t }),
                    None => ()
                }
            },
            10
        )
    };

    let toggle = Callback::<MouseEvent>::from(
        {
            let timer_time = timer_time.clone();
            let start_time = start_time.clone();
            move |_| {
                match *start_time {
                    Some(t) => {
                        let done_time = DisplayTime { millis: chrono::Utc::now().timestamp_millis() - t };
                        timer_time.set(done_time.clone());
                        start_time.set(None);
                        done_callback.emit(done_time.clone());
                    },
                    None => {
                        start_time.set( Some(chrono::Utc::now().timestamp_millis()));
                        timer_time.set(DisplayTime::new());
                    }
                };
            }
        }
    );
    html! {
        <div onclick={toggle}>{timer_time.display()}</div>
    }
}
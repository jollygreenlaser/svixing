use crate::client_utils::ErrorDisplay;
use crate::error_template::{AppError, ErrorTemplate};
use crate::types::{BarTask, BazTask, CreateTask, FooTask, TaskStatus};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="FooBar"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(endpoint = "add_task")]
pub async fn add_task(task: CreateTask, delay: u32) -> Result<(), ServerFnError> {
    println!("Sees task: {task:?} with delay: {delay}");
    Ok(())
}

#[component]
fn HomePage() -> impl IntoView {
    let foo_id = create_rw_signal(None);
    let delay_seconds = create_rw_signal(0);

    let add_task_action = create_action(move |task: &CreateTask| {
        let task = task.clone();
        async move {
            add_task(task, delay_seconds.get_untracked()).await?;
            // series_data.refetch();
            Ok(())
        }
    });

    view! {
    <div>
        <h1>"Task Manager"</h1>
        <ErrorDisplay res=add_task_action />
        <div>
            <input prop:value=delay_seconds
                    on:input=move |ev| delay_seconds.set(event_target_value(&ev).parse().unwrap_or_default()) />
            <span>Delay Seconds</span>
        </div>
        <div>
            <input prop:value=move || foo_id().unwrap_or_default()
                on:input=move |ev| foo_id.set(Some(event_target_value(&ev)).filter(|v| !v.is_empty())) />
            <button disabled=move || add_task_action.pending()() || foo_id().is_none()
                on:click=move |_| add_task_action.dispatch(CreateTask::Foo(foo_id().unwrap_or_default()))>
                Add Foo With ID: {move || foo_id()}
            </button>
        </div>
        <button disabled=move || add_task_action.pending()()
            on:click=move |_| add_task_action.dispatch(CreateTask::Bar)>
            Add Bar
        </button>
        <button disabled=move || add_task_action.pending()()
            on:click=move |_| add_task_action.dispatch(CreateTask::Baz)>
            Add Baz
        </button>
    </div>
    }
}

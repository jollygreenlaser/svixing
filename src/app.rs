use crate::client_utils::ErrorDisplay;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/svixbar.css"/>
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CreateTask {
    Foo(String),
    Bar,
    Baz,
}

#[server(endpoint = "add_task")]
pub async fn add_task(task: CreateTask) -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
fn HomePage() -> impl IntoView {
    let foo_id = create_rw_signal(None);

    let add_task_action = create_action(move |task: &CreateTask| {
        let task = task.clone();
        async move {
            add_task(task).await?;
            // series_data.refetch();
            Ok(())
        }
    });

    view! {
        <h1>"Task Manager"</h1>
        <ErrorDisplay res=add_task_action />
        <div>
            <input id="foo_id" name="foo_id"
                prop:value=foo_id on:input=move |ev| foo_id.set(Some(event_target_value(&ev)).filter(|v| !v.is_empty())) />
            <button disabled=move || add_task_action.pending()() || foo_id().is_none()
                on:click=move |_| add_task_action.dispatch(CreateTask::Foo(foo_id().unwrap_or_default()))>
                Add Foo With ID: {move || foo_id()}
            </button>
        </div>
    }
}

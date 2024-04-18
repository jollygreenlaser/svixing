use crate::client_utils::ErrorDisplay;
use crate::error_template::{AppError, ErrorTemplate};
use crate::types::{AllTasks, BarTask, BazTask, CreateTask, FooTask, TaskStatus};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::server_utils::db_client;
use crate::types::ItemId;
}
}

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

#[server(endpoint = "get_tasks")]
pub async fn get_tasks() -> Result<AllTasks, ServerFnError> {
    Ok(db_client()
        .query_required_single(
            "select {
                        foo := (select FooTask {
                            status: { * },
                            given_id,
                        }),
                        bar := (select BarTask {
                            status: { * },
                            status_code,
                        }),
                        baz := (select BazTask {
                            status: { * },
                            rand_num,
                        })
                    }",
            &(),
        )
        .await?)
}

#[server(endpoint = "add_task")]
pub async fn add_task(task: CreateTask, delay: i32) -> Result<(), ServerFnError> {
    println!("Sees task: {task:?} with delay: {delay}");

    let status_insert = "(insert TaskStatus {
        execute_after := datetime_of_statement() + to_duration(seconds := <int32>$0),
    })";

    // Exposing a weakness of the Rust edgedb bindings - it doesn't yet handle named args
    // Thus the tasks with no input need a dummy binding
    let (input, query) = match task {
        CreateTask::Foo(given_id) => (
            Some(given_id),
            format!(
                "insert FooTask {{
                    status := {status_insert},
                    given_id := <str>$1
                }}"
            ),
        ),
        CreateTask::Bar => (
            None,
            format!(
                "with dummy := <optional str>$1 insert BarTask {{ status := {status_insert} }}"
            ),
        ),
        CreateTask::Baz => (
            None,
            format!(
                "with dummy := <optional str>$1 insert BazTask {{ status := {status_insert} }}"
            ),
        ),
    };

    let _: ItemId = db_client().query_required_single(&query, &(input,)).await?;

    Ok(())
}

#[component]
fn HomePage() -> impl IntoView {
    let foo_id = create_rw_signal(None);
    let delay_seconds = create_rw_signal(0);

    let task_data = create_resource(|| (), |_| async move { get_tasks().await });

    let add_task_action = create_action(move |task: &CreateTask| {
        let task = task.clone();
        async move {
            add_task(task, delay_seconds.get_untracked()).await?;
            task_data.refetch();
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
        <button on:click=move |_| task_data.refetch()>
            Refresh Tasks
        </button>
        <Suspense
            fallback=move || view! { <div></div> }
        >
            {move || task_data.get().map(|res| match res {
                Ok(AllTasks { foo, bar, baz }) => {
                    view! {
                        <h1>Foo Tasks</h1>

                        <h1>Bar Tasks</h1>

                        <h1>Baz Tasks</h1>
                    }.into_view()
                }
                Err(err) => format!("Could not fetch tasks: {err:?}").into_view(),
            })}
        </Suspense>
    </div>
    }
}

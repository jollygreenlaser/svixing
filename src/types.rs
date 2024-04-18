use leptos::*;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
use edgedb_derive::Queryable;

#[derive(Queryable, Debug)]
pub struct ItemId {
    pub id: Uuid,
}

}
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskStatus {
    pub created_at_str: String,
    pub execute_after_str: String,
    pub started_at_str: Option<String>,
    pub finished_at_str: Option<String>,
}

pub const STATUS_FIELDS: &str = "status := .status {
    created_at_str := <str>.created_at,
    execute_after_str := <str>.execute_after,
    started_at_str := <str>.started_at,
    finished_at_str := <str>.finished_at,
}";

impl IntoView for TaskStatus {
    fn into_view(self) -> View {
        let TaskStatus {
            created_at_str,
            execute_after_str,
            started_at_str,
            finished_at_str,
        } = self;
        view! {
            <p>Status Metadata</p>
            <p>Created at: {created_at_str}</p>
            <p>Execute at: {execute_after_str}</p>
            <p>Started at: {started_at_str.unwrap_or("Not started".to_string())}</p>
            <p>Finished at: {finished_at_str.unwrap_or("Not finished".to_string())}</p>
        }
        .into_view()
    }
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FooTask {
    pub id: Uuid,
    pub status: TaskStatus,
    pub given_id: String,
}

impl IntoView for FooTask {
    fn into_view(self) -> View {
        let FooTask {
            id,
            status,
            given_id,
        } = self;
        view! {
            <p>ID: {id.to_string()}</p>
            <p>Given ID: {given_id}</p>
            {status}
        }
        .into_view()
    }
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BarTask {
    pub id: Uuid,
    pub status: TaskStatus,
    pub status_code: Option<i32>, // u32 may be more semantically correct, but edgedb only has i32
}

impl IntoView for BarTask {
    fn into_view(self) -> View {
        let BarTask {
            id,
            status,
            status_code,
        } = self;
        view! {
            <p>ID: {id.to_string()}</p>
            <p>Status code: {status_code.map(|sc| sc.to_string()).unwrap_or("Not run".to_string())}</p>
            {status}
        }
        .into_view()
    }
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BazTask {
    pub id: Uuid,
    pub status: TaskStatus,
    pub rand_num: Option<i32>,
}

impl IntoView for BazTask {
    fn into_view(self) -> View {
        let BazTask {
            id,
            status,
            rand_num,
        } = self;
        view! {
            <p>ID: {id.to_string()}</p>
            <p>Random number: {rand_num.map(|rn| rn.to_string()).unwrap_or("Not run".to_string())}</p>
            {status}
        }
        .into_view()
    }
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AllTasks {
    pub foo: Vec<FooTask>,
    pub bar: Vec<BarTask>,
    pub baz: Vec<BazTask>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CreateTask {
    Foo(String),
    Bar,
    Baz,
}

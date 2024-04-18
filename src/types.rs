use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
use edgedb_protocol::model::Uuid;
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
    pub created_at: String,
    pub execute_after: String,
    pub started_at: String,
    pub finished_at: String,
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FooTask {
    pub status: TaskStatus,
    pub given_id: String,
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BarTask {
    pub status: TaskStatus,
    pub status_code: i32, // u32 may be more semantically correct, but edgedb only has i32
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BazTask {
    pub status: TaskStatus,
    pub rand_num: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CreateTask {
    Foo(String),
    Bar,
    Baz,
}

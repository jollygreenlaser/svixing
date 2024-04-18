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
    pub id: Uuid,
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
    pub status_code: Option<i32>, // u32 may be more semantically correct, but edgedb only has i32
}

#[cfg_attr(feature = "ssr", derive(edgedb_derive::Queryable))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BazTask {
    pub status: TaskStatus,
    pub rand_num: Option<i32>,
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

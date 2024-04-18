cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {

use crate::types::ItemId;
use crate::server_utils::db_client;
use edgedb_protocol::model::Uuid;
use edgedb_derive::Queryable;
use tokio::time::sleep;
use std::time::Duration;

#[derive(Debug, Clone, Queryable)]
struct FooTask {
    id: Uuid,
    status_id: Uuid,
    given_id: String,
}

fn make_query(task_name: &str, shape: &str) -> String {
    // This is not a good query because this data model was wrong. Whoops.
    format!("select (
        update (
            select {task_name}
            filter .status.execute_after < datetime_of_statement()
            and not exists .status.started_at
        ).status 
        set {{
            started_at := datetime_of_statement()
        }}
    ).<status[is {task_name}] {{
        id,
        status_id := .status.id,
        {shape}
    }}")
}

pub fn start_worker() {
    // Really regretting messing up my data model. For now, three separate workers!

    // TODO: Handle detecting tasks started but not finished with excessive delay
    // TODO: Proper batching so that it's not just doing everything available at once
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;


            let query = make_query("FooTask", "given_id");
            let tasks: Vec<FooTask> = match db_client().query(&query, &()).await {
                Ok(tasks) => tasks,
                Err(err) => {
                    println!("Failed to fetch foos: {err:?}");
                    continue;
                },
            };

            if !tasks.is_empty() {
                println!("Spawning tasks for foos: {tasks:?}");
                tokio::spawn(async move {
                    sleep(Duration::from_millis(3000)).await;
                    println!("Doing foo tasks: {}", tasks.iter().map(|foo| foo.given_id.clone()).collect::<Vec<String>>().join(", "));
                    match db_client().query_json("update TaskStatus
                            filter .id in array_unpack(<array<uuid>>$0)
                            set {
                                finished_at := datetime_of_statement()
                            }", &(tasks.iter().map(|foo| foo.status_id).collect::<Vec<Uuid>>(),)).await {
                        Ok(_) => (),
                        Err(err) => {
                            println!("Failed to update foos: {err:?}");
                        },
                    };
                });
            }

        }
    });

    // tokio::spawn(async move {
    //     loop {
    //         let tasks
    //     }
    // });

    // tokio::spawn(async move {
    //     loop {
    //         let tasks
    //     }
    // });
}

}
}

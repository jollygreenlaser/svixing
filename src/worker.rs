cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::server_utils::db_client;
use edgedb_protocol::model::Uuid;
use edgedb_derive::Queryable;
use edgedb_tokio::Client;
use tokio::time::sleep;
use std::time::Duration;
use reqwest::Client as ReqClient;
use serde::Serialize;
use futures::{stream, StreamExt};


#[derive(Debug, Clone, Queryable)]
struct FooTask {
    id: Uuid,
    status_id: Uuid,
    given_id: String,
}

#[derive(Debug, Clone, Queryable, Serialize)]
struct BarTask {
    id: Uuid,
    status_id: Uuid,
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

async fn do_bar(BarTask { id, status_id }: BarTask, req: &ReqClient, db: &Client) {
    let status_code = match req
        .get("https://www.xkcd.com") // Was only getting 400's on the time URL
        .send()
        .await {
            Ok(res) => res.status().as_u16() as i32,
            Err(err) => {
                println!("Failed to request bar: {err}");
                return;
            }
        };

    println!("Bar task [{id}] got status code: {status_code}");

    match db.query_json("select {
            st := (
                update TaskStatus
                filter .id = <uuid>$0
                set {
                    finished_at := datetime_of_statement()
                }
            ),
            bar := (
                update BarTask
                filter .id = <uuid>$1
                set {
                    status_code := <int32>$2
                }
            )
        }", &(status_id, id, status_code)).await {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to update bar: {err:?}");
        },
    };
}

pub fn start_worker() {
    let db = db_client();
    // Really regretting messing up my data model. For now, three separate workers!

    // TODO: Handle detecting tasks started but not finished with excessive delay
    // TODO: Proper batching so that it's not just doing everything available at once
    // TODO: Retries
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;


            let query = make_query("FooTask", "given_id");
            let tasks: Vec<FooTask> = match db.query(&query, &()).await {
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
                    match db.query_json("update TaskStatus
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

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;

            let query = make_query("BarTask", "");
            let tasks: Vec<BarTask> = match db.query(&query, &()).await {
                Ok(tasks) => tasks,
                Err(err) => {
                    println!("Failed to fetch bars: {err:?}");
                    continue;
                },
            };

            if !tasks.is_empty() {
                println!("Spawning tasks for bars: {tasks:?}");
                tokio::spawn(async move {
                    // TODO: Maybe make req static?
                    let req = ReqClient::new();
                    stream::iter(tasks.into_iter())
                        .map(|bar| do_bar(bar, &req, &db))
                        .buffer_unordered(4) // Sufficiently arbitrary magic number
                        .collect::<Vec<()>>()
                        .await;
                });
            }
        }
    });

    // Baz tasks are left as an exercise to the reader
}

}
}

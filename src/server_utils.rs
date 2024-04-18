cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
// Copy/pasted utils from my other codebases
use edgedb_protocol::model::Uuid;
use edgedb_tokio::{Builder, Client};
use std::{
    sync::OnceLock,
    time::{Duration, SystemTime},
};

static EDGEDB: OnceLock<Client> = OnceLock::new();

async fn init_db() {
    let alt = Builder::new()
        .instance(env!("DB_INSTANCE"))
        .unwrap()
        .build_env()
        .await.unwrap();

    EDGEDB.get_or_init(|| Client::new(&alt));
}

pub fn db_client<'a>() -> &'a Client {
    EDGEDB.get().unwrap()
}

}
}

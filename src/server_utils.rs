cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
// Copy/pasted utils from my other codebases
use edgedb_tokio::{Builder, Client};
use std::{
    sync::OnceLock,
};

static EDGEDB: OnceLock<Client> = OnceLock::new();

pub async fn init_db() {
    let alt = Builder::new()
        .instance("svixedb") // Would env var in prod
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

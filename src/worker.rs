cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {

pub fn start_worker() {
    tokio::spawn(async move {
        println!("Hello there");
    });
}

}
}

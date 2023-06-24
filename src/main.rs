pub mod environment;
mod dal {
    pub mod crud_ops;
    pub mod models {
        pub mod task;
    }
}
mod bll {
    pub mod logic;
}
mod ui {
    pub mod presentation;
}
use dal::crud_ops;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();
    ui::presentation::run().await.unwrap();
}

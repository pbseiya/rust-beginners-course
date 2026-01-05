mod config;
mod models;
mod services;

fn main() {
    config::init();
    models::user::show_user();
    models::product::show_product();
    services::auth::login();
    services::auth::logout();
    services::payment::process_payment(100.015);
    println!("✅ Application finished successfully.");
}

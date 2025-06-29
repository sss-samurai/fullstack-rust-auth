pub mod main_template;
pub mod sidebar;
pub mod router;  
pub mod profile;  
pub mod routes;
pub mod settings;
pub mod home;
pub mod utils;
pub mod login_and_signin;
pub mod api_hook;
pub mod function_hook;
pub mod config;
pub mod types;
// Optional: Re-export if you want to use it as `components::MainTemplate`
pub use main_template::MainTemplate;

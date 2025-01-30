use crate::data::person::User;
use sqlx::{MySqlPool};
fn register_user(username: &str, plain_password: &str, pool: &MySqlPool) -> Result<(), String> {
   Ok(()) 
}

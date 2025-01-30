use crate::data::person::User;
fn register_user(username: &str, plain_password: &str) -> Result<(), String> {
    User::get_username(username, pool);

   Ok(()) 
}

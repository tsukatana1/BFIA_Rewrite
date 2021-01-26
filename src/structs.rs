use sqlx::FromRow;
use rocket::FromForm;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    //display_name: String,
    //real_name: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Clone, FromRow)]
pub struct SignedUser {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(FromForm, Debug)]
pub struct RegisterForm {
    username: String,
    display_name: String,
    real_name: String,
    email: String,
    passwd: String,
    telephone: String,
    gender: String,
    donate_i: String
}

#[derive(Debug)]
struct Admin {
    user: User
}
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
    pub username: String,
    pub display_name: String,
    pub real_name: String,
    pub email: String,
    pub passwd: String,
    pub telephone: String,
    pub description: String,
    pub gender: String,
    pub donate_i: String
}

#[derive(Debug)]
struct Admin {
    user: User
}
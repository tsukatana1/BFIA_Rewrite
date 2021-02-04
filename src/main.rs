#![feature(proc_macro_hygiene, decl_macro)]

/*
    MESSAGE TO FUTURE DEVS OF THE OFFICAL BFIA WEBPAGE:
    Please note that Rust is the best language to write a webserver is. Rust is a DDP language, which is really fast
    and really good for large scale projects. Speed and safety is essential, and Rust provides it without much hassle.
    Previously, it was made with express.js, but JavaScript is one of the worst languages yet.
*/

//imports
mod structs;

use structs::{User, SignedUser, RegisterForm};

use rocket::{
    uri,
    launch,
    request::{Form, FromRequest, Outcome},
    response::{NamedFile, status::NotFound, Redirect, Flash},
    get,
    post,
    routes,
    catch,
    fairing::{Fairing, AdHoc},
    State,
    Rocket,
    Request,
    FromForm,
};

use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    collections::HashMap,
    convert::Infallible,
};

use rocket_contrib::{
    templates::Template,
    json::Json,
};

use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    FromRow,
    query_as
};

use serde::{Deserialize, Serialize};

use bcrypt::{hash_with_salt, DEFAULT_COST, verify};

//constants

//traits/implementations

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for &'a SignedUser {
    type Error = Infallible;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {

        let user_result = request.local_cache_async(async {

            let db = request.managed_state::<PgPool>().unwrap();

            let user_id = request.cookies().get_private("user_id");

            match user_id {
                Some(cookie) => {
                    let uuid = cookie.value().parse::<i64>().ok();
                    let username = request.cookies().get_private("username").unwrap().value().parse::<String>().ok();
                    let password = request.cookies().get_private("password").unwrap().value().parse::<String>().ok();

                    let user = sqlx::query_as!(SignedUser, "SELECT user_id, username, password FROM accounts WHERE user_id = $1 AND username = $2 AND password = $3", uuid, username, password)
                        .fetch_one(db)
                        .await;

                    Some(user)
                },
                None => {
                    println!("No cookies.");

                    return None
                }
            }
        }).await;

        match user_result.as_ref() {
            Some(user) => {
                return Outcome::Success(user.as_ref().unwrap());
            },
            None => return Outcome::Forward(()),
        };
    }
} 

//functions

#[get("/backend/pkg/<file..>")]
async fn wasm(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("backend/pkg/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/css/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("css/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/")]
async fn home() -> Template {
    let mut context = HashMap::new();
    context.insert("Donors".to_string(), "1".to_string());
    Template::render("index", &context)
}

#[get("/home")]
async fn home_route() -> Template {
    let mut context = HashMap::new();
    context.insert("Donors".to_string(), "1".to_string());
    Template::render("home", &context)
}

#[get("/dashboard")]
async fn dashboard_route(_state: State<'_, PgPool>, user: &SignedUser) -> Template {
    let mut context = HashMap::new();
    context.insert("Dashboard".to_string(), "Welcome".to_string());

    Template::render("dashboard", &context)
}

#[get("/login")]
async fn login() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();

    Template::render("login", &context)
}

#[get("/register")]
async fn register() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();

    Template::render("register", &context)
}


//post routes

#[post("/new_reg", data = "<user_input>")]
async fn newuser(state: State<'_, PgPool>, user_input: Form<RegisterForm>) {
    /* sqlx::query!("INSERT INTO accounts(display_name, username, real_name) VALUES($1, $2, $3)", user_input.display_name, user_input.username, user_input.real_name)
    .execute(&mut state)
    .await
    .unwrap(); */
    
    /* let hashe = hash_with_salt(&user_input.passwd, DEFAULT_COST, &[
        38, 113, 212, 141, 108, 213, 195, 166, 201, 38, 20, 13, 47, 40, 104, 18,
    ]).unwrap().to_string(); */
} 

//errors

#[launch]
async fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Connecting to db", |rocket| async {
            let pg_url = "postgres://postgres:encrypted01@localhost:5432/BakFamily".to_string();

            let pool = PgPoolOptions::new()
                .max_connections(20)
                .connect(&pg_url)
                .await;

            match(pool) {
                Ok(p) => Ok(rocket.manage(p)),
                Err(why) => {
                    println!("{:?}", why);
                    Err(rocket)
                }
            }
        }))
        .mount("/", routes![home, files, wasm, home_route, dashboard_route, login, register, newuser])
}

//postgres://postgres:encrypted01@localhost:5432/
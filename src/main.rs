#![feature(proc_macro_hygiene, decl_macro)]

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
async fn newuser(user_input: Form<RegisterForm>) -> Flash<Redirect> {
    println!("{:?}", user_input);
    Flash::success(Redirect::to("/"), "Nice job")
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
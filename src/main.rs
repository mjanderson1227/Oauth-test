mod auth;
use dotenv::dotenv;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::{get, launch, routes, uri, Build, Rocket};

#[get("/")]
fn root() -> &'static str {
    "Hello user!"
}

#[get("/login")]
fn login_screen(_cookies: &CookieJar<'_>) -> Redirect {
    let oauth_client = auth::OAuthClient::new();
    let (auth_url, _csrf_token) = oauth_client.gen_auth_url();
    Redirect::to(auth_url.to_string())
}

#[get("/books")]
fn books_menu(cookies: &CookieJar) {}

/*
* Some guidelines:
* 1. I want an interface such that when the user logs in, they can see the books they have created.
*  - Query the database and find the user info and serve it to them.
* 2. I want to be able to reroute a user to the login screen if they are not already logged in.
*  - Add a guard such that when the user does not meet the logged in criteria they are rerouted to
*  the login screen
* */
#[launch]
fn rocket() -> Rocket<Build> {
    // Parse the .env file
    dotenv().ok();
    rocket::build().mount("/", routes![root, login_screen])
}

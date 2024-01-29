mod auth;
use dotenv::dotenv;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{get, launch, routes, Build, Rocket};

#[get("/")]
fn root() -> &'static str {
    "Hello user!"
}

#[get("/login")]
fn login_screen(cookies: &CookieJar<'_>) -> Redirect {
    let oauth_client = auth::OAuthClient::new();
    let (auth_url, csrf_token) = oauth_client.gen_auth_url();
    let csrf_cookie = Cookie::build(("oauth_csrf", csrf_token.secret().to_owned()))
        .path("/callback")
        .build();
    cookies.add(csrf_cookie);
    Redirect::to(auth_url.to_string())
}

#[get("/callback?<state>&<code>")]
fn login_handler(state: String, code: String, cookies: &CookieJar<'_>) -> String {
    format!(
        "csrf: {}\nauth_key: {}\nCookies: {:?}",
        state, code, cookies
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Parse the .env file
    dotenv().ok();
    rocket::build().mount("/", routes![root, login_screen, login_handler])
}

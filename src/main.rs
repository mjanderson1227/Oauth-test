mod auth;
use dotenv::dotenv;
use rocket::http::{Cookie, CookieJar, SameSite};
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
    /* TODO: Fix this later as it could be unsafe. (Eventually move over to using private cookies)*/
    let csrf_cookie = Cookie::build(("oauth_csrf", csrf_token.secret().to_owned()))
        .path("/callback")
        .same_site(SameSite::Lax)
        .build();
    cookies.add(csrf_cookie);
    Redirect::to(auth_url.to_string())
}

#[get("/callback?<state>&<code>")]
fn login_handler(state: String, code: String, cookies: &CookieJar<'_>) -> Redirect {
    let cookie_result = cookies.get("oauth_csrf");
    if cookie_result.is_some_and(|cookie| cookie.value().to_string() == state) {
        cookies.add(
            // TODO: Move this over to a private cookie.
            Cookie::build(("oauth_token", code))
                .same_site(SameSite::Lax)
                .build(),
        );
        Redirect::to("/authenticate")
    } else {
        Redirect::to("/login")
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Parse the .env file.
    dotenv().ok();
    // Launch the server.
    rocket::build().mount("/", routes![root, login_screen, login_handler])
}

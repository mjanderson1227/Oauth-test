mod auth;
use dotenv::dotenv;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::{get, launch, routes, Build, Rocket, State};

#[get("/")]
fn root() -> &'static str {
    "Hello user!"
}

#[get("/login")]
fn login_screen(oauth_client: &State<auth::OAuthClient>, cookies: &CookieJar<'_>) -> Redirect {
    let (auth_url, csrf_token, pkce_verifier) = oauth_client.gen_auth_url();
    /* TODO: Fix this later as it could be unsafe. (Eventually move over to using private cookies)*/
    let csrf_cookie = Cookie::build(("oauth_csrf", csrf_token.secret().to_owned()))
        .path("/callback")
        .same_site(SameSite::Lax)
        .build();
    let pkce_cookie = Cookie::build(("oauth_pkce", pkce_verifier.secret().to_owned()))
        .same_site(SameSite::Lax)
        .build();
    cookies.add(csrf_cookie);
    cookies.add(pkce_cookie);
    Redirect::to(auth_url.to_string())
}

#[get("/callback?<state>&<code>")]
fn login_handler(state: String, code: String, cookies: &CookieJar<'_>) -> Redirect {
    if cookies
        .get("oauth_csrf")
        .is_some_and(|cookie| cookie.value().to_string() == state)
    {
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

#[get("/authenticate")]
async fn authenticate(
    oauth_client: &State<auth::OAuthClient>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    let cookie_list = vec![cookies.get("oauth_pkce"), cookies.get("oauth_token")];
    let strs: Vec<Option<String>> = cookie_list
        .iter()
        .map(|cookie| cookie.map(|coo| coo.value().to_string()))
        .collect();

    if let (Some(pkce), Some(token)) = (&strs[0], &strs[1]) {
        let auth_token = oauth_client.verify_token(token, pkce).await;
        match auth_token {
            Ok(token) => {
                cookies
                    .add(Cookie::build(("oauth_access_token", token.secret().to_string())).build());
                Redirect::to("/final")
            }
            Err(_) => Redirect::to("/login"),
        }
    } else {
        Redirect::to("/login")
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Parse the .env file.
    dotenv().ok();
    // Launch the server.
    rocket::build().manage(auth::OAuthClient::new()).mount(
        "/",
        routes![root, login_screen, login_handler, authenticate],
    )
}

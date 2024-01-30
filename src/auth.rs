use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use std::env::var;

pub struct OAuthClient {
    client: BasicClient,
}

impl OAuthClient {
    pub fn new() -> Self {
        let client = BasicClient::new(
            ClientId::new(var("CLIENT_ID").expect("Unable to find client id.\n")),
            Some(ClientSecret::new(
                var("CLIENT_SECRET").expect("Unable to find secret key.\n"),
            )),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Unable to acquire auth url.\n"),
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                    .expect("Invalid token URL\n"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://127.0.0.1:8000/callback".to_string())
                .expect("Invalid redirect URL\n"),
        );

        Self { client }
    }

    pub fn gen_auth_url(self: Self) -> (Url, CsrfToken) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        println!("verifier: {:?}\n", pkce_verifier);
        self.client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/cloud-platform.read-only".to_string(),
            ))
            .url()
    }
}

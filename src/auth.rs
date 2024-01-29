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
        let client_id = ClientId::new(var("CLIENT_ID").expect("Unable to find client id.\n"));
        let client_secret = Some(ClientSecret::new(
            var("CLIENT_SECRET").expect("Unable to find secret key.\n"),
        ));
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Unable to acquire auth url.\n");
        let token_url = Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid token URL\n"),
        );
        let redirect = RedirectUrl::new("http://127.0.0.1:8000/callback".to_string())
            .expect("Invalid token URL");
        let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
            .set_redirect_uri(redirect);
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

    pub fn validate_auth_token() {}
}

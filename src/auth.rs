use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::env::var;

#[derive(Debug, Clone)]
pub struct TokenNotFoundError;

impl std::fmt::Display for TokenNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A token was not found.\n")
    }
}

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

    pub fn gen_auth_url(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (link, csrf) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/cloud-platform.read-only".to_string(),
            ))
            .url();
        (link, csrf, pkce_verifier)
    }

    pub async fn verify_token(
        &self,
        token: &String,
        verifier: &String,
    ) -> Result<AccessToken, TokenNotFoundError> {
        let response_result = self
            .client
            .exchange_code(AuthorizationCode::new(token.to_string()))
            .set_pkce_verifier(PkceCodeVerifier::new(verifier.to_string()))
            .request_async(async_http_client)
            .await;
        if let Ok(res) = response_result {
            Ok(res.access_token().to_owned())
        } else {
            Err(TokenNotFoundError)
        }
    }
}

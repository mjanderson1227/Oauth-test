use std::fmt;

use rocket::request::FromRequest;

#[derive(Debug, Clone)]
struct UserNotFoundError;

impl fmt::Display for UserNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User not found.\n")
    }
}

struct User;

impl<'r> FromRequest<'r> for User {
    type Error = UserNotFoundError;

    fn from_request<'life0, 'async_trait>(
        request: &'r rocket::Request<'life0>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = rocket::request::Outcome<Self, Self::Error>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
    }
}

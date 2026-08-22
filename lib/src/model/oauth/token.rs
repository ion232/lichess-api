use crate::model::{Body, Request};
use serde::Serialize;

/// Form used to exchange an authorization code for an access token.
///
/// The `code` comes from the redirect back to your `redirect_uri`, and
/// `code_verifier` must be the value the `code_challenge` was derived from.
/// Both `redirect_uri` and `client_id` must match those used to request the
/// authorization code.
#[derive(Clone, Debug, Serialize)]
pub struct TokenExchangeForm {
    grant_type: &'static str,
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String,
    pub client_id: String,
}

impl TokenExchangeForm {
    pub fn new(
        code: impl Into<String>,
        code_verifier: impl Into<String>,
        redirect_uri: impl Into<String>,
        client_id: impl Into<String>,
    ) -> Self {
        Self {
            grant_type: "authorization_code",
            code: code.into(),
            code_verifier: code_verifier.into(),
            redirect_uri: redirect_uri.into(),
            client_id: client_id.into(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, TokenExchangeForm>;

impl PostRequest {
    pub fn new(form: TokenExchangeForm) -> Self {
        Self::post("/api/token", None, Body::Form(form), None)
    }
}

impl From<TokenExchangeForm> for PostRequest {
    fn from(form: TokenExchangeForm) -> Self {
        Self::new(form)
    }
}

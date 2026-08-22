use crate::error::Result;
use crate::model::Domain;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Parameters for the OAuth2 authorization endpoint.
///
/// This endpoint is not called by this library: it renders an authorization
/// prompt for the user in a browser, and the result is delivered as query
/// parameters appended to your `redirect_uri`. Use [`AuthorizationUrl::to_url`]
/// to build the URL to send the user to.
///
/// `response_type` and `code_challenge_method` are fixed by the spec and are
/// set for you.
///
/// The flow uses PKCE. Generate two random strings unique to each authorization
/// request, a `code_verifier` and a `state`, and store them for the duration of
/// the request. The `code_challenge` below is `BASE64URL(SHA256(code_verifier))`;
/// this library does not compute it, to avoid pulling in cryptographic
/// dependencies. Keep the `code_verifier` out of URLs and off insecure
/// connections.
///
/// When the user is redirected back, check that the returned `state` matches
/// the one you generated, then exchange the returned `code` for an access token
/// with [`super::token::TokenExchangeForm`].
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct AuthorizationUrl {
    response_type: &'static str,
    /// Arbitrary identifier that uniquely identifies your application.
    pub client_id: String,
    /// The absolute URL the user should be redirected to with the result.
    pub redirect_uri: String,
    code_challenge_method: &'static str,
    /// `BASE64URL(SHA256(code_verifier))`.
    pub code_challenge: String,
    /// Space separated list of requested OAuth scopes, if any.
    pub scope: Option<String>,
    /// Hint that the user should log in with a specific Lichess username.
    pub username: Option<String>,
    /// Arbitrary state returned verbatim with the authorization result.
    pub state: Option<String>,
}

impl AuthorizationUrl {
    pub fn new(
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        code_challenge: impl Into<String>,
    ) -> Self {
        Self {
            response_type: "code",
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
            code_challenge_method: "S256",
            code_challenge: code_challenge.into(),
            scope: None,
            username: None,
            state: None,
        }
    }

    /// Space separated list of requested OAuth scopes.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Hint that the user should log in with a specific Lichess username.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Arbitrary state returned verbatim with the authorization result.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Build the URL to send the user to in order to grant authorization.
    pub fn to_url(&self) -> Result<url::Url> {
        let base_url = format!("https://{}", Domain::Lichess.as_ref());
        let mut url = url::Url::parse(&base_url).expect("invalid base url");

        {
            let mut query_pairs = url.query_pairs_mut();
            let query_serializer = serde_urlencoded::Serializer::new(&mut query_pairs);
            self.serialize(query_serializer)?;
        }

        url.set_path("/oauth");

        Ok(url)
    }
}

use super::PendingAuthorization;
use super::pkce::{Pkce, generate_state};
use crate::error::Result;
use crate::model::Domain;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Parameters for the OAuth2 authorization endpoint.
///
/// This endpoint is not called by this library: it renders an authorization
/// prompt for the user in a browser, and the result is delivered as query
/// parameters appended to your `redirect_uri`.
///
/// `response_type` and `code_challenge_method` are fixed by the spec and are
/// set for you.
///
/// # Example
///
/// ```no_run
/// use lichess_api::model::oauth::authorize::AuthorizationUrl;
///
/// # fn main() -> lichess_api::error::Result<()> {
/// let (url, pending) = AuthorizationUrl::generated("example.com", "http://example.com/")
///     .scope("preference:read")
///     .start()?;
///
/// // Send the user to `url`, and keep `pending` until they are redirected back.
/// # Ok(())
/// # }
/// ```
///
/// [`AuthorizationUrl::start`] generates the PKCE secrets and the `state` for
/// you, and returns a [`PendingAuthorization`] that verifies the result and
/// completes the exchange. Use [`AuthorizationUrl::new`] with
/// [`AuthorizationUrl::to_url`] only if you are managing those secrets
/// yourself, and keep the `code_verifier` out of URLs and off insecure
/// connections.
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
    /// Start an authorization request whose PKCE secrets and `state` are
    /// generated for you by [`AuthorizationUrl::start`].
    ///
    /// Prefer this over [`AuthorizationUrl::new`] unless you are managing the
    /// PKCE secrets yourself.
    pub fn generated(client_id: impl Into<String>, redirect_uri: impl Into<String>) -> Self {
        Self::new(client_id, redirect_uri, String::new())
    }

    /// Build an authorization request from a `code_challenge` you computed
    /// yourself.
    ///
    /// The challenge is `BASE64URL(SHA256(code_verifier))`; see
    /// [`Pkce::derive_challenge`]. Most callers should use
    /// [`AuthorizationUrl::generated`] with [`AuthorizationUrl::start`] instead.
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

    /// Begin an authorization request, generating the PKCE secrets and `state`
    /// for you.
    ///
    /// Returns the URL to send the user to, and a [`PendingAuthorization`]
    /// holding the secrets needed to complete the flow. Store the pending value
    /// for the duration of the request (in session storage for a web backend,
    /// in memory for a native app) and finish with
    /// [`PendingAuthorization::complete`].
    ///
    /// Any `state` set on this builder is replaced by a freshly generated one.
    /// Use [`PendingAuthorization::new`] directly if you must supply your own.
    pub fn start(mut self) -> Result<(url::Url, PendingAuthorization)> {
        let pkce = Pkce::generate();
        let state = generate_state();

        self.code_challenge = pkce.challenge().to_string();
        self.state = Some(state.clone());

        let url = self.to_url()?;
        let pending =
            PendingAuthorization::new(pkce.verifier(), state, self.client_id, self.redirect_uri);

        Ok((url, pending))
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

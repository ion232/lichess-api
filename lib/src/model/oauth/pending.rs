use super::token::TokenExchangeForm;
use crate::error::{Error, Result};

/// The secrets needed to complete an authorization request.
///
/// Created by [`super::authorize::AuthorizationUrl::start`]. Hold this for the
/// duration of the flow — in session storage for a web backend, in memory for a
/// native or client-side app — then finish with
/// [`PendingAuthorization::complete`].
///
/// Keeping the verifier and state together here means the CSRF check and the
/// verifier cannot be forgotten: the only way to reach the token exchange is
/// through a method that performs both.
///
/// # Example
///
/// ```no_run
/// use lichess_api::client::LichessApi;
/// use lichess_api::model::oauth::authorize::AuthorizationUrl;
///
/// # async fn run() -> lichess_api::error::Result<()> {
/// let (url, pending) = AuthorizationUrl::generated("example.com", "http://example.com/")
///     .scope("preference:read")
///     .start()?;
///
/// // Send the user to `url`. They come back to your `redirect_uri`, which
/// // carries the authorization result in its query string.
/// let redirect_url = url::Url::parse("http://example.com/?code=...&state=...").unwrap();
///
/// // No token yet, so the client is unauthenticated here.
/// let api = LichessApi::new(reqwest::Client::new(), None);
/// let token = pending.complete(&api, &redirect_url).await?;
///
/// // Subsequent requests act on behalf of the user.
/// let api = LichessApi::new(reqwest::Client::new(), Some(token.access_token));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct PendingAuthorization {
    verifier: String,
    state: String,
    client_id: String,
    redirect_uri: String,
}

impl PendingAuthorization {
    pub fn new(
        verifier: impl Into<String>,
        state: impl Into<String>,
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            verifier: verifier.into(),
            state: state.into(),
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
        }
    }

    /// The `state` this request expects back from the authorization result.
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Parse an authorization result and produce the token exchange form.
    ///
    /// `redirect_url` is the full URL the user was redirected back to,
    /// including its query string. Returns an error if the authorization was
    /// denied, if the `state` does not match, or if the URL is missing the
    /// authorization code.
    ///
    /// Use this when you want to inspect or send the exchange yourself;
    /// [`PendingAuthorization::complete`] does this and performs the exchange.
    pub fn exchange_form(self, redirect_url: &url::Url) -> Result<TokenExchangeForm> {
        let mut code = None;
        let mut state = None;
        let mut error = None;
        let mut error_description = None;

        for (key, value) in redirect_url.query_pairs() {
            match key.as_ref() {
                "code" => code = Some(value.into_owned()),
                "state" => state = Some(value.into_owned()),
                "error" => error = Some(value.into_owned()),
                "error_description" => error_description = Some(value.into_owned()),
                _ => {}
            }
        }

        // Check state before anything else, so a forged redirect is rejected
        // regardless of what it carries.
        //
        // A failed authorization returns the state too, so this is verifiable
        // even on the error path. Treat a missing state as a mismatch.
        let returned_state = state.unwrap_or_default();
        if !constant_time_eq(returned_state.as_bytes(), self.state.as_bytes()) {
            return Err(Error::OAuthStateMismatch);
        }

        if let Some(error) = error {
            return Err(Error::OAuth {
                error,
                error_description,
            });
        }

        let code = code.ok_or_else(|| {
            Error::Response("authorization result has neither a code nor an error".to_string())
        })?;

        Ok(TokenExchangeForm::new(
            code,
            self.verifier,
            self.redirect_uri,
            self.client_id,
        ))
    }

    /// Complete the flow: verify the authorization result and exchange the code
    /// for an access token.
    ///
    /// `redirect_url` is the full URL the user was redirected back to.
    ///
    /// The client need not be authenticated — this is what produces the token —
    /// so `LichessApi::new(client, None)` is fine here.
    pub async fn complete(
        self,
        api: &crate::client::LichessApi<reqwest::Client>,
        redirect_url: &url::Url,
    ) -> Result<super::AccessToken> {
        let form = self.exchange_form(redirect_url)?;
        api.obtain_access_token(form).await
    }
}

/// Compare two byte strings without short-circuiting on the first difference.
///
/// The state is a CSRF token, so its comparison should not leak how much of a
/// guess was correct through timing.
fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    left.iter()
        .zip(right)
        .fold(0u8, |acc, (l, r)| acc | (l ^ r))
        == 0
}

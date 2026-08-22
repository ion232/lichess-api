use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngExt;
use sha2::{Digest, Sha256};

/// Number of random bytes used for generated secrets.
///
/// 32 bytes base64url-encodes to 43 characters, the minimum length RFC 7636
/// permits for a `code_verifier`.
const SECRET_BYTES: usize = 32;

/// Generate a cryptographically random, base64url-encoded secret.
fn generate_secret() -> String {
    let bytes: [u8; SECRET_BYTES] = rand::rng().random();
    URL_SAFE_NO_PAD.encode(bytes)
}

/// A PKCE secret pair, as described by RFC 7636.
///
/// The `verifier` is kept private until the token exchange; the `challenge` is
/// what gets sent in the authorization request. Only the challenge travels over
/// the initial redirect, so an eavesdropper who intercepts the authorization
/// code cannot exchange it without the verifier.
///
/// Keep the verifier out of URLs and off insecure connections. For fully
/// client-side apps the user themselves can always extract it, which is fine.
#[derive(Clone, Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
}

impl Pkce {
    /// Generate a new random verifier and its derived challenge.
    pub fn generate() -> Self {
        let verifier = generate_secret();
        let challenge = Self::derive_challenge(&verifier);

        Self {
            verifier,
            challenge,
        }
    }

    /// `BASE64URL(SHA256(code_verifier))`, the `S256` challenge method.
    pub fn derive_challenge(verifier: &str) -> String {
        URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
    }

    /// The secret sent only in the token exchange.
    pub fn verifier(&self) -> &str {
        &self.verifier
    }

    /// The derived value sent in the authorization request.
    pub fn challenge(&self) -> &str {
        &self.challenge
    }
}

/// Generate a random `state` value, used to tie an authorization result back to
/// the request that started it and defend against cross site request forgery.
pub fn generate_state() -> String {
    generate_secret()
}

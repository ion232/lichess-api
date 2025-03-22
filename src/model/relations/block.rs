use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new(username: &str) -> Self {
        Self::post(format!("/api/rel/block/{username}"), None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(username: S) -> Self {
        Self::new(username.as_ref())
    }
}

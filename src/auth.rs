use crate::vars::{USER_AGENT, github_api_url};
use crate::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET};
use reqwest::header;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use std::collections::HashMap;
use std::convert::Infallible;

pub(crate) struct GitHub;

pub(crate) type Auth = rocket_oauth2::OAuth2<GitHub>;
pub(crate) type TokenResponse = rocket_oauth2::TokenResponse<GitHub>;

#[derive(Debug)]
pub(crate) enum Owner {
    Success,
    Failure,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Owner {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req
            .cookies()
            .get_private("token")
            .map(|c| c.value().to_string())
            .unwrap_or_else(String::new);

        let mut map = HashMap::new();
        map.insert("access_token", token);

        let url = github_api_url() + "/token";
        let client = reqwest::Client::new();
        let status = client
            .post(url)
            .basic_auth(GITHUB_CLIENT_ID, Some(GITHUB_CLIENT_SECRET))
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header(header::USER_AGENT, USER_AGENT)
            .json(&map)
            .send()
            .await
            .unwrap()
            .status();

        if status.is_success() {
            Outcome::Success(Owner::Success)
        } else {
            Outcome::Success(Owner::Failure)
        }
    }
}

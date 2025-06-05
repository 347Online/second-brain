use dotenv_codegen::dotenv;

pub(crate) const ROCKET_SECRET_KEY: &'static str = dotenv!("ROCKET_SECRET_KEY");

pub(crate) const GITHUB_CLIENT_ID: &'static str = dotenv!("GITHUB_CLIENT_ID");
pub(crate) const GITHUB_CLIENT_SECRET: &'static str = dotenv!("GITHUB_CLIENT_SECRET");

pub(crate) const USER_AGENT: &'static str = concat!(
    "Mozilla/5.0 (Compatible); ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "; +https://brain.347online.me;"
);

pub(crate) fn github_api_url() -> String {
    format!("https://api.github.com/applications/{}", GITHUB_CLIENT_ID)
}

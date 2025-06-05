mod auth;
mod routes;
mod thought;
mod vars;

use self::auth::Auth;
use self::routes::{capture, github_callback, github_login, index};
use self::vars::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, ROCKET_SECRET_KEY};
use rocket::fs::{FileServer, relative};
use rocket::{Config, launch, routes};

#[launch]
fn rocket() -> _ {
    let figment = Config::figment()
        .merge(("secret_key", ROCKET_SECRET_KEY))
        .merge(("oauth.github.provider", "GitHub"))
        .merge(("oauth.github.client_id", GITHUB_CLIENT_ID))
        .merge(("oauth.github.client_secret", GITHUB_CLIENT_SECRET));

    rocket::build()
        .configure(figment)
        .mount("/", routes![github_callback, github_login, index, capture])
        .attach(Auth::fairing("github"))
        .mount("/static", FileServer::from(relative!("static")))
}

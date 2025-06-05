use crate::auth::{Auth, Owner, TokenResponse};
use crate::thought::Thought;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use rocket::{get, post};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[get("/login/github")]
pub(crate) fn github_login(oauth2: Auth, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/auth/github")]
pub(crate) fn github_callback(token: TokenResponse, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build(),
    );
    Redirect::to("/")
}

#[get("/")]
pub(crate) async fn index(auth: Owner) -> Result<NamedFile, Redirect> {
    match auth {
        Owner::Success => Ok(NamedFile::open("index.html").await.unwrap()),
        Owner::Failure => Err(Redirect::to("/login/github")),
    }
}

#[post("/", data = "<input>")]
pub(crate) fn capture(auth: Owner, input: Form<Thought>) -> Result<Redirect, Status> {
    match auth {
        Owner::Success => match save_thought(input.into_inner()) {
            Ok(_) => Ok(Redirect::to("/")),
            Err(e) => {
                eprintln!("An error occurred: {}", e);
                Err(Status::InternalServerError)
            }
        },

        Owner::Failure => Err(Status::Forbidden),
    }
}

fn save_thought(thought: Thought) -> Result<(), Box<dyn Error>> {
    let fname = format!("notes/{}.md", thought.title());
    let mut file = File::create_new(fname)?;
    file.write(thought.description().as_bytes())?;

    Ok(())
}

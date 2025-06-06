use rocket::FromForm;

#[derive(FromForm)]
pub(crate) struct Thought<'r> {
    title: &'r str,
    description: &'r str,
}

impl<'r> Thought<'r> {
    pub fn title(&self) -> &'r str {
        self.title.trim()
    }

    pub fn description(&self) -> &'r str {
        self.description.trim()
    }

    pub fn dissolve(self) -> (&'r str, &'r str) {
        (self.title(), self.description())
    }
}

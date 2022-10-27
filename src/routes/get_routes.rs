pub mod get {
    //to format response to html
    use rocket::response::content;

    #[get("/")]
    pub fn index() -> content::RawHtml<&'static str> {
        content::RawHtml("\
        <h1>Hello from the back side</h1>\
        ")
    }

    #[get("/salutation")]
    pub fn salutation() -> content::RawText<&'static str> {
    content::RawText("another salutation")
    }
}
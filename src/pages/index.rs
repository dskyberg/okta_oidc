use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
pub struct IndexPage {
    pub id: String,
    pub url: String,
    pub label: String,
    pub userinfo: String,
}
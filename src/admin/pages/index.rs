use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "admin/index.html")] // using the template in this path, relative
pub struct Index {
    pub scopes: Vec<String>,
    pub claims: Vec<String>,
}

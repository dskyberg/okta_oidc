use std::collections::HashMap;

use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "app/index.html")] // using the template in this path, relative
pub struct Index {
    pub id: String,
    pub url: String,
    pub label: String,
    pub claims: HashMap<String, String>,
}

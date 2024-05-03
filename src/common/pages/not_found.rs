use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "common/not_found.html")] // using the template in this path, relative
pub struct NotFoundPage {}

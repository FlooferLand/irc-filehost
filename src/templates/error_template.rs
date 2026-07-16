const TEMPLATE: &str = include_str!("../../templates/error.html");

/// Doesn't use Askama for reliability reasons
pub struct ErrorTemplate {
    pub name: String,
    pub text: String
}
impl ErrorTemplate {
    pub fn render(self) -> String {
        TEMPLATE
            .replace("{{name}}", &self.name)
            .replace("{{text}}", &self.text)
    }
}

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct FrontMatter {
    pub template: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
}

pub fn extract_front_matter(content: &str) -> (Option<FrontMatter>, &str) {
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let front_matter = &content[3..end + 3];
            let rest = &content[end + 6..];
            let fm: FrontMatter = serde_yaml::from_str(front_matter).unwrap_or_default();
            return (Some(fm), rest);
        }
    }
    (None, content)
}
use super::RelationType;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkDirective {
    url: Url,
    rel: Option<RelationType>,
    rev: Option<RelationType>,
    anchor: Option<String>,
}

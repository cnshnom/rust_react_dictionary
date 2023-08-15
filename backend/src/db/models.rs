use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Queryable)]
pub struct WordPair{
    pub id: String,
    pub german: String,
    pub chinese: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CreateWordPair{
    pub german: String,
    pub chinese: String
}

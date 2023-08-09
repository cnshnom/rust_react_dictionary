use super::schema::word_pairs;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Queryable)]
pub struct WordPair{
    pub id: String,
    pub german: String,
    pub chinese: String
}

#[derive(Insertable)]
#[diesel(table_name=word_pairs)]
pub struct NewWordPair<'a> {
    pub id: &'a str,
    pub german: &'a str,
    pub chinese: &'a str
}
#[derive(Deserialize)]
#[derive(Debug)]
pub struct CreateWordPair{
    pub german: String,
    pub chinese: String
}
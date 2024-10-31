use kernel::model::book::{Book, event::CreateBook};
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from (value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        Slef {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from (value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
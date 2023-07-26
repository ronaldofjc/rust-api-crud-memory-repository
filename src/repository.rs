use actix_web::web::{Data, Json, Path};
use uuid::Uuid;
use crate::AppState;
use crate::entity::{Book, BookRequest, ErrorData};

pub async fn get_all(data: Data<AppState>) -> Vec<Book> {
    let books = data.books.lock().unwrap();
    books.clone()
}

pub async fn get_by_id(id: Path<Uuid>, data: Data<AppState>) -> Result<Option<Book>, ErrorData> {
    let books = data.books.lock().unwrap();
    let book_repo = books.iter().find(|book| book.id == id.to_owned());
    if book_repo.is_none() {
        return Err(ErrorData::new("book not found".to_string()));
    }

    Ok(book_repo.cloned())
}

pub async fn remove_by_id(id: Path<Uuid>, data: Data<AppState>) -> Result<Option<Book>, ErrorData> {
    let mut books = data.books.lock().unwrap();
    let book_repo = books.iter_mut().position(|book| book.id == id.to_owned());
    if let Some(index) = book_repo {
        let removed_book = books.remove(index);
        return Ok(Some(removed_book))
    }

    return Err(ErrorData::new("book not found".to_string()));
}

pub async fn update_by_id(id: Path<Uuid>, body: Json<BookRequest>, data: Data<AppState>) -> Result<Option<Book>, ErrorData> {
    let mut books = data.books.lock().unwrap();

    let book = books.iter_mut().find(|book| book.id == id.to_owned());
    if book.is_none() {
        return Err(ErrorData::new("book not found".to_string()));
    }

    let book = book.unwrap();
    let payload = Book {
        id: book.id.to_owned(),
        title: body.title.to_owned().unwrap_or(book.title.clone()),
        author: body.author.to_owned().unwrap_or(book.author.clone()),
    };

    *book = payload;
    return Ok(Some(book.clone()))
}

pub async fn create(payload: Json<BookRequest>, data: Data<AppState>) -> Result<Option<Book>, ErrorData> {
    let mut books = data.books.lock().unwrap();
    let book = Book {
        id: Uuid::new_v4(),
        title: payload.title.clone().unwrap(),
        author: payload.author.clone().unwrap(),
    };

    books.push(book.clone());
    return Ok(Some(book.clone()))
}
use axum::{routing::get, routing::post, Router};
use serde::{Serialize, Deserialize};
use axum::Json;

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32
}

#[derive(Serialize)]
struct BookResponse {
    book: Book,
    summary: String,
    is_long: bool
}

impl Book {
    fn summary(&self) -> String {
        return format!("Title by {}, {} pages", self.author, self.pages);
    }

    fn is_long(&self) -> bool {
        return self.pages > 300;
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book", get(get_book))
        .route("/book", post(post_book));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
        
    println!("Сервер запущен на http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


async fn get_book() -> Json<BookResponse> {
    let book = Book{
        title: String::from("Little Prince"),
        author: String::from("Dont Remember"),
        pages: 286
    };

    let summary = book.summary();
    let is_long = book.is_long();

    return Json(
        BookResponse{
            book,
            summary,
            is_long
        }
    )
}

async fn post_book(Json(book): Json<Book>) -> Json<BookResponse> {
    let summary = book.summary();
    let is_long = book.is_long();

    return Json(
        BookResponse{
            book,
            summary,
            is_long
        }
    )
}

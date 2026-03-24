# Rust HTTP server

I was learning the language. This project is a practice of writing own stuff with Rust

Build:
```bash
cargo build
```

Run:
```bash
cargo run
```

Server will launch at [localhost:3000](http://localhost:3000)

---

## Available endpoints

- `GET /book`
```bash
om1ji@Mac web-server % curl http://localhost:3000/book
{
  "book":
    {
      "title": "Little Prince",
      "author": "Exupery",
      "pages": 286
    },
  "summary": "Title by Dont Remember, 286 pages",
  "is_long": false
}                      
```

- `POST /book`
```bash
om1ji@Mac web-server % curl -X POST http://localhost:3000/book \            
  -H "Content-Type: application/json" \
  -d '{"title":"1984","author":"George Orwell", "pages":564}'

{
  "book":
    {
      "title": "1984",
      "author": "George Orwell",
      "pages": 564
    },
  "summary": "Title by George Orwell, 564 pages",
  "is_long": true
}
```

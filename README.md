# Demo REST API

A simple CRUD REST API built with Rust, Actix Web, and PostgreSQL.

## Stack

- Rust
- Actix Web
- SQLx
- PostgreSQL
- Repository / Service pattern

## Project Structure

```text
src/
├── main.rs
├── db/
├── errors/
├── handlers/
├── models.rs
├── repositories/
└── services/

db/
└── migration/
    └── 001_init.sql
```

## Run manual

```bash
cargo run
```

## Run Docker Compose
```bash
docker compose up --build
```

Default server address:

```text
http://127.0.0.1:8080
```

## Endpoints

| Method | Path | Description |
| --- | --- | --- | --- |
| `GET` | `/` | Hello / health check |
| `GET` | `/users` | Get all users from database |
| `GET` | `/users/{id}` | Get one user from database by UUID |
| `POST` | `/users` | Create a user in database |
| `DELETE` | `/users/{id}` | Delete a user in database by UUID |

## API Demo

### `GET /`

Request:

```bash
curl --location 'http://localhost:8080'
```

Example response:

```json
{
  "message": "Hello from Rust API!"
}
```

### `GET /users`

Request:

```bash
curl --location 'http://localhost:8080/users'
```

Example response:

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Alice",
    "email": "alice@example.com",
    "created_at": "2026-03-13T10:00:00Z"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Bob",
    "email": "bob@example.com",
    "created_at": "2026-03-13T10:05:00Z"
  }
]
```

### `GET /users/{id}`

Request:

```bash
curl --location 'http://localhost:8080/users/9171e83d-a54f-406b-863a-2a0795330faa'
```

Example response:

```json
{
    "id": "9171e83d-a54f-406b-863a-2a0795330faa",
    "name": "John Smith",
    "email": "john.smith@gmail.com",
    "created_at": "2026-03-13T08:11:17.909096Z"
}
```

### `POST /users`

Request:

```bash
curl --location 'http://localhost:8080/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "id": 1,
    "name": "len",
    "email": "len@gmail.com"
}'
```

Example response:

```json
{
    "id": "99017acf-6c57-4dae-aba6-8945c1f461a7",
    "name": "len",
    "email": "len@gmail.com",
    "created_at": "2026-03-13T08:16:41.540279Z"
}
```

### `DELETE /users/{id}`

Request:

```bash
curl --location --request DELETE 'http://localhost:8080/users/9171e83d-a54f-406b-863a-2a0795330faa'
```

Example response:

```text
204 No Content
```

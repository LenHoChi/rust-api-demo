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

## Run

```bash
cargo run
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
curl http://127.0.0.1:8080/
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
curl http://127.0.0.1:8080/users
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
curl http://127.0.0.1:8080/users/550e8400-e29b-41d4-a716-446655440000
```

Example response:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Alice",
  "email": "alice@example.com",
  "created_at": "2026-03-13T10:00:00Z"
}
```

### `POST /users`

Request:

```bash
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Charlie",
    "email": "charlie@example.com"
  }'
```

Example response:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "name": "Charlie",
  "email": "charlie@example.com",
  "created_at": "2026-03-13T10:10:00Z"
}
```

### `DELETE /users/{id}`

Request:

```bash
curl -X DELETE http://127.0.0.1:8080/users/550e8400-e29b-41d4-a716-446655440000
```

Example response:

```text
204 No Content
```

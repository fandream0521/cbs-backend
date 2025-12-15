# Quickstart: Admin Backend API with SQLite Logging

## Prerequisites

- Rust toolchain (stable, >= latest as of 2025-12-15)
- SQLite available (file DB path: `./.cms_backend/cms_backend.db`)

## Setup

1. Install dependencies:
   ```bash
   cargo fetch
   ```

2. Create data directory (database file will be created automatically):
   ```bash
   mkdir -p ./.cms_backend
   ```

3. Run migrations:
   ```bash
   cargo run --bin migrate
   ```
   
   This will create all tables and indexes in `./.cms_backend/cms_backend.db`.

## Run

```bash
cargo run
```

- Service starts on default port (typically `http://localhost:3000`).
- Service exposes HTTP API per `specs/api.md`.
- CORS allows all origins.

### Example: Login and Get Token

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"name": "admin", "password": "admin123"}'
```

Response:
```json
{
  "code": 200,
  "message": "成功",
  "data": {
    "token": "your-bearer-token-here"
  }
}
```

### Example: Access Protected Endpoint

```bash
curl -X GET http://localhost:3000/users/list \
  -H "Authorization: Bearer your-bearer-token-here" \
  -H "Content-Type: application/json" \
  -d '{"offset": 0, "size": 10, "name": ""}'
```

## Test

```bash
cargo test
```

- Unit tests cover each handler/service success+failure paths.
- Integration tests exercise HTTP endpoints with token auth.
- Tests use temporary SQLite databases for isolation.

### Run Specific Test Suites

```bash
# Run all unit tests
cargo test --lib

# Run all integration tests
cargo test --test mod

# Run specific integration test
cargo test --test mod test_story_create_and_list
```

## Observability

- **Request Logging**: Logs include request ID, method/path/status/latency.
- **SQL Logging**: SQL logs include statement, params, rows/affected.
- **Tracing**: Structured tracing spans around database queries and handlers.

## API Endpoints

### Authentication
- `POST /login` - Login and get bearer token
- `GET /test` - Validate token
- `GET /health` - Health check (no auth required)

### System Management (US1)
- `POST /users` - Create user
- `GET /users/:id` - Get user
- `PATCH /users/:id` - Update user
- `DELETE /users/:id` - Delete user
- `POST /users/list` - List users with pagination
- Similar endpoints for `/departments`, `/roles`, `/menus`

### Goods Management (US2)
- `PATCH /goods` - Create goods
- `GET /goods/:id` - Get goods
- `PATCH /goods/:id` - Update goods
- `DELETE /goods/:id` - Delete goods
- `POST /goods/list` - List goods with pagination
- Similar endpoints for `/category`

### Story & Analytics (US3)
- `POST /story` - Create story
- `POST /story/list` - List stories
- `POST /menu/tree` - Get menu tree
- `GET /role/:role_id/menu` - Get role menu tree
- `GET /role/:role_id/menuIds` - Get role menu IDs
- `POST /role/assign` - Assign menus to role
- `GET /goods/category/count` - Category counts
- `GET /goods/category/sale` - Category sales
- `GET /goods/category/favor` - Category favors
- `GET /goods/sale/top10` - Top 10 sales
- `GET /goods/address/sale` - Address sales
- `GET /goods/amount/list` - Goods amount list

All endpoints (except `/login`, `/test`, `/health`) require Bearer token authentication.


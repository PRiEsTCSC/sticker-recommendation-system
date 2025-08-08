# Sticker Recommendation System

The **Sticker Recommendation System** is a Rust-based backend API built with the Actix Web framework and PostgreSQL. It provides sticker recommendations based on user interactions, featuring user and admin management, JWT-based authentication, and a versioned API under `/v1`. This project is beginner-friendly, with built-in logging, rate limiting, and CORS support for seamless frontend integration.

## Why Use This Project?

- **Fast Web Server**: Powered by Actix Web with logging and rate limiting (30 requests/second, burst size 30).
- **Database Ready**: Uses PostgreSQL with SQLx for reliable data storage (`users`, `admins`, `sessions`, `interactions`, `sticker_metrics`).
- **Secure Authentication**: JWT-based authentication with 24-hour token expiration.
- **CORS Support**: Configurable for browser-based APIs via `FRONTEND_URL`.
- **Docker Support**: Easy PostgreSQL setup with Docker.
- **Quick Testing**: Includes a `/v1/health` endpoint to verify server status.

## What You’ll Need

- **[Rust](https://www.rust-lang.org/tools/install)** (stable):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Verify: `rustc --version`
- **[PostgreSQL](https://www.postgresql.org/download/)** or **[Docker](https://www.docker.com/get-started)**
- **[curl](https://curl.se/download.html)** (for command-line testing)
- **[jq](https://stedolan.github.io/jq/download/)** (optional, for JSON parsing)
- **[Postman](https://www.postman.com/downloads/)** (optional, for GUI-based testing)

## Step-by-Step Setup

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/sticker-recommendation-system.git
cd sticker-recommendation-system/backend-rust
```

### 2. Configure Your Environment

Create a `.env` file in the `backend-rust` directory:

```plaintext
DATABASE_URL=postgresql://sticker_user:securepassword@localhost:5432/sticker_db
JWT_SECRET=your_jwt_secret_here_32_bytes_long
FRONTEND_URL=http://localhost:3000  # Optional, for CORS
```

Generate a secure `JWT_SECRET`:

```bash
openssl rand -base64 32
```

Add it to `.env`. Ensure `.env` is listed in `.gitignore`.

### 3. Set Up PostgreSQL

**Using Docker (recommended)**:

```bash
docker run -d --name postgres \
  -e POSTGRES_USER=sticker_user \
  -e POSTGRES_PASSWORD=securepassword \
  -e POSTGRES_DB=sticker_db \
  -p 5432:5432 postgres:16
```

Verify PostgreSQL is running:

```bash
docker ps
```

**Alternatively, install PostgreSQL locally**:

```bash
psql -U postgres -c "CREATE USER sticker_user WITH PASSWORD 'securepassword';"
psql -U postgres -c "CREATE DATABASE sticker_db OWNER sticker_user;"
```

### 4. Install Dependencies

```bash
cargo build
```

This downloads and compiles dependencies (e.g., `actix-web`, `sqlx`, `jsonwebtoken`, `bcrypt`).

### 5. Run the Application

```bash
cargo run
```

- The server starts at `http://localhost:8080` (configurable in `configs/env_load.rs`).
- The database schema is initialized automatically.
- Test the server:

```bash
curl http://localhost:8080/v1/health
```

**Expected output**: `"Server is healthy"`

### 6. Configure Logging

Ensure a `log4rs.yaml` file exists in the project root:

```yaml
appenders:
  stdout:
    kind: console
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
  file:
    kind: file
    path: "log/app.log"
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
root:
  level: info
  appenders:
    - stdout
    - file
```

Logs are written to `log/app.log` and the console.

## API Endpoints

**Base URL**: `http://localhost:8080/v1`

### Health Check

- **GET /v1/health**
  - **Description**: Check server status.
  - **Response**: `"Server is healthy"`

### Authentication

- **POST /v1/auth/register/user**
  - **Request**:
    ```json
    {"username": "testuser", "password": "mypassword"}
    ```
  - **Response**:
    ```json
    {"token": "eyJ...", "username": "testuser"}
    ```

- **POST /v1/auth/login/user**
  - **Request**:
    ```json
    {"username": "testuser", "password": "mypassword"}
    ```
  - **Response**:
    ```json
    {"token": "eyJ...", "username": "testuser"}
    ```

- **POST /v1/auth/register/admin**
  - **Request**:
    ```json
    {"username": "testadmin", "password": "adminpass"}
    ```
  - **Response**:
    ```json
    {"token": "eyJ...", "username": "testadmin"}
    ```

- **POST /v1/auth/login/admin**
  - **Request**:
    ```json
    {"username": "testadmin", "password": "adminpass"}
    ```
  - **Response**:
    ```json
    {"token": "eyJ...", "username": "testadmin"}
    ```

### User Routes (Protected)

- **PUT /v1/user/update-username**
  - **Authentication**: User JWT
  - **Request**:
    ```json
    {"new_username": "newuser"}
    ```
  - **Response**:
    ```json
    {"username": "newuser"}
    ```

- **DELETE /v1/user/delete**
  - **Authentication**: User JWT
  - **Response**: `"User deleted"`

### Admin Routes (Protected)

- **GET /v1/admin/users**
  - **Authentication**: Admin JWT
  - **Response**:
    ```json
    [{"id": "uuid", "username": "testuser", "password_hash": "..."}, ...]
    ```

- **POST /v1/admin/users**
  - **Authentication**: Admin JWT
  - **Request**:
    ```json
    {"username": "newuser", "password": "newpass"}
    ```
  - **Response**:
    ```json
    {"username": "newuser"}
    ```

- **PUT /v1/admin/users/{id}**
  - **Authentication**: Admin JWT
  - **Request**:
    ```json
    {"username": "updateduser", "password": "updatedpass"}
    ```
  - **Response**:
    ```json
    {"username": "updateduser"}
    ```

- **DELETE /v1/admin/users/{id}**
  - **Authentication**: Admin JWT
  - **Response**: `"User deleted"`

## Curl Commands

Test endpoints using `curl`. Replace `<user_token>`, `<admin_token>`, and `<user_id>` with values from login responses or `GET /v1/admin/users`.

### 1. Health Check

```bash
curl -X GET http://localhost:8080/v1/health
```

### 2. Register a User

```bash
curl -X POST http://localhost:8080/v1/auth/register/user \
     -H "Content-Type: application/json" \
     -d '{"username":"testuser","password":"mypassword"}'
```

### 3. Login as a User

```bash
curl -X POST http://localhost:8080/v1/auth/login/user \
     -H "Content-Type: application/json" \
     -d '{"username":"testuser","password":"mypassword"}'
```

Copy the `token` for user routes.

### 4. Register an Admin

```bash
curl -X POST http://localhost:8080/v1/auth/register/admin \
     -H "Content-Type: application/json" \
     -d '{"username":"testadmin","password":"adminpass"}'
```

### 5. Login as an Admin

```bash
curl -X POST http://localhost:8080/v1/auth/login/admin \
     -H "Content-Type: application/json" \
     -d '{"username":"testadmin","password":"adminpass"}'
```

Copy the `token` for admin routes.

### 6. Update Username (User)

```bash
curl -X PUT http://localhost:8080/v1/user/update-username \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <user_token>" \
     -d '{"new_username":"newuser"}'
```

### 7. Delete User (User)

```bash
curl -X DELETE http://localhost:8080/v1/user/delete \
     -H "Authorization: Bearer <user_token>"
```

### 8. List All Users (Admin)

```bash
curl -X GET http://localhost:8080/v1/admin/users \
     -H "Authorization: Bearer <admin_token>"
```

### 9. Add a User (Admin)

```bash
curl -X POST http://localhost:8080/v1/admin/users \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <admin_token>" \
     -d '{"username":"newuser2","password":"newpass"}'
```

### 10. Update a User (Admin)

```bash
curl -X PUT http://localhost:8080/v1/admin/users/<user_id> \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <admin_token>" \
     -d '{"username":"updateduser","password":"updatedpass"}'
```

### 11. Delete a User (Admin)

```bash
curl -X DELETE http://localhost:8080/v1/admin/users/<user_id> \
     -H "Authorization: Bearer <admin_token>"
```

### Automating Token Extraction

Use `jq` to extract tokens:

```bash
USER_TOKEN=$(curl -s -X POST http://localhost:8080/v1/auth/login/user \
     -H "Content-Type: application/json" \
     -d '{"username":"testuser","password":"mypassword"}' | jq -r '.token')
curl -X PUT http://localhost:8080/v1/user/update-username \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $USER_TOKEN" \
     -d '{"new_username":"newuser"}'
```

## Postman Guide

### 1. Install Postman

Download [Postman](https://www.postman.com/downloads/).

### 2. Create a Collection

1. Open Postman and click **New > Collection**.
2. Name it `Sticker Recommendation System`.
3. Set a collection variable:
   - **Variable**: `base_url`
   - **Value**: `http://localhost:8080/v1`

### 3. Add Requests

Add these requests to the collection:

#### Health Check
- **Method**: GET
- **URL**: `{{base_url}}/health`
- **Save as**: `Health Check`

#### Register User
- **Method**: POST
- **URL**: `{{base_url}}/auth/register/user`
- **Headers**: `Content-Type: application/json`
- **Body** (raw, JSON):
  ```json
  {"username": "testuser", "password": "mypassword"}
  ```
- **Save as**: `Register User`

#### Login User
- **Method**: POST
- **URL**: `{{base_url}}/auth/login/user`
- **Headers**: `Content-Type: application/json`
- **Body**:
  ```json
  {"username": "testuser", "password": "mypassword"}
  ```
- **Tests**:
  ```javascript
  pm.collectionVariables.set("user_token", pm.response.json().token);
  ```
- **Save as**: `Login User`

#### Register Admin
- **Method**: POST
- **URL**: `{{base_url}}/auth/register/admin`
- **Headers**: `Content-Type: application/json`
- **Body**:
  ```json
  {"username": "testadmin", "password": "adminpass"}
  ```
- **Save as**: `Register Admin`

#### Login Admin
- **Method**: POST
- **URL**: `{{base_url}}/auth/login/admin`
- **Headers**: `Content-Type: application/json`
- **Body**:
  ```json
  {"username": "testadmin", "password": "adminpass"}
  ```
- **Tests**:
  ```javascript
  pm.collectionVariables.set("admin_token", pm.response.json().token);
  ```
- **Save as**: `Login Admin`

#### Update Username (User)
- **Method**: PUT
- **URL**: `{{base_url}}/user/update-username`
- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer {{user_token}}`
- **Body**:
  ```json
  {"new_username": "newuser"}
  ```
- **Save as**: `Update Username`

#### Delete User (User)
- **Method**: DELETE
- **URL**: `{{base_url}}/user/delete`
- **Headers**: `Authorization: Bearer {{user_token}}`
- **Save as**: `Delete User`

#### List Users (Admin)
- **Method**: GET
- **URL**: `{{base_url}}/admin/users`
- **Headers**: `Authorization: Bearer {{admin_token}}`
- **Save as**: `List Users`

#### Add User (Admin)
- **Method**: POST
- **URL**: `{{base_url}}/admin/users`
- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer {{admin_token}}`
- **Body**:
  ```json
  {"username": "newuser2", "password": "newpass"}
  ```
- **Save as**: `Add User (Admin)`

#### Update User (Admin)
- **Method**: PUT
- **URL**: `{{base_url}}/admin/users/{{user_id}}`
- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer {{admin_token}}`
- **Body**:
  ```json
  {"username": "updateduser", "password": "updatedpass"}
  ```
- **Save as**: `Update User (Admin)`

#### Delete User (Admin)
- **Method**: DELETE
- **URL**: `{{base_url}}/admin/users/{{user_id}}`
- **Headers**: `Authorization: Bearer {{admin_token}}`
- **Save as**: `Delete User (Admin)`

### 4. Test the Collection

1. Run `Register User` or `Login User` to set `user_token`.
2. Run `Register Admin` or `Login Admin` to set `admin_token`.
3. Run protected requests, ensuring tokens are set.
4. For admin routes, get `user_id` from `List Users`.

### 5. Export Collection

Export for sharing:

1. Click the collection > **Export** > Save as JSON.
2. Share `Sticker Recommendation System.postman_collection.json`.

## What's Included in the Project?

| File/Folder                       | Purpose                                                                 |
|-----------------------------------|-------------------------------------------------------------------------|
| `src/main.rs`                     | Entry point, initializes logging and starts the server.                  |
| `src/init.rs`                     | Configures Actix Web with logging, rate limiting, CORS, and routes.      |
| `src/configs/env_load.rs`         | Loads `DATABASE_URL`, `JWT_SECRET`, `FRONTEND_URL` from `.env`.          |
| `src/structs/database_structs.rs` | Defines database structs and connection logic using SQLx.                |
| `src/routes/health.rs`            | Defines `GET /v1/health` endpoint.                                       |
| `src/routes/auth.rs`              | Handles JWT authentication and registration/login endpoints.             |
| `src/routes/user.rs`              | User management endpoints (`/v1/user/*`).                                |
| `src/routes/admin.rs`             | Admin management endpoints (`/v1/admin/*`).                              |
| `src/middleware/`                 | JWT validation and CORS middleware.                                     |
| `log4rs.yaml`                     | Configures logging output to console and `log/app.log`.                  |

## Customizing Your Project

### Adding New Endpoints

1. Create a new file in `src/routes/` (e.g., `interactions.rs`):

```rust
use actix_web::{post, web, HttpResponse, Responder};
use crate::structs::database_structs::DatabaseConnection;

#[post("/interactions")]
async fn add_interaction(db: web::Data<DatabaseConnection>, req: web::Json<InteractionRequest>) -> impl Responder {
    db.save_interaction(req.user_id, &req.input_text, &req.detected_emotion, &req.sticker_url).await
        .map(|_| HttpResponse::Ok().body("Interaction saved"))
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Error: {}", e)))
}
```

2. Update `src/routes/mod.rs`:

```rust
pub mod health;
pub mod auth;
pub mod user;
pub mod admin;
pub mod interactions;
```

3. Add to `src/init.rs` under `/v1` scope:

```rust
.configure(routes::interactions::init_routes)
```

4. Test:

```bash
curl -X POST http://localhost:8080/v1/interactions \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <user_token>" \
     -d '{"user_id":"uuid","input_text":"Hello","detected_emotion":"happy","sticker_url":"http://example.com/sticker.png"}'
```

### Adding Database Queries

1. In `src/structs/database_structs.rs`, add a function:

```rust
pub async fn get_interactions(&self, user_id: Uuid) -> Result<Vec<Interaction>, sqlx::Error> {
    sqlx::query_as::<_, Interaction>("SELECT * FROM interactions WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
}
```

2. Create a route in `src/routes/interactions.rs`:

```rust
#[get("/interactions/{user_id}")]
async fn get_interactions(db: web::Data<DatabaseConnection>, path: web::Path<String>) -> impl Responder {
    let user_id = Uuid::parse_str(&path).map_err(|_| HttpResponse::BadRequest().body("Invalid user ID"))?;
    db.get_interactions(user_id).await
        .map(|interactions| HttpResponse::Ok().json(interactions))
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Error: {}", e)))
}
```

3. Register in `src/init.rs` and test:

```bash
curl -X GET http://localhost:8080/v1/interactions/<user_id> \
     -H "Authorization: Bearer <user_token>"
```

### Changing Settings

- **Logging**: Modify `log4rs.yaml` for verbosity:

```yaml
root:
  level: debug
```

- **Rate Limiting**: Adjust in `src/init.rs`:

```rust
let governor = GovernorConfigBuilder::default()
    .seconds_per_request(60)
    .burst_size(10)
    .finish()
    .unwrap();
```

- **CORS**: Restrict origins in `src/middleware/cors_mgt.rs`:

```rust
Cors::default().allowed_origin("https://yourfrontend.com")
```

## Troubleshooting

- **“Database connection failed”**
  - Verify `DATABASE_URL` in `.env`.
  - Ensure PostgreSQL is running:
    ```bash
    docker ps
    ```
  - Test connectivity:
    ```bash
    psql -U sticker_user -d sticker_db
    ```

- **“Invalid token”**
  - Ensure `JWT_SECRET` matches the one used to generate tokens.
  - Check token expiration (24 hours). Re-run login:
    ```bash
    curl -X POST http://localhost:8080/v1/auth/login/user \
         -H "Content-Type: application/json" \
         -d '{"username":"testuser","password":"mypassword"}'
    ```
  - Verify session in database:
    ```bash
    docker exec -it postgres psql -U sticker_user -d sticker_db -c "SELECT * FROM sessions;"
    ```

- **“Port already in use”**
  - Change `PORT` in `.env` (e.g., `8081`).
  - Stop conflicting processes:
    ```bash
    killall cargo
    ```

- **“Build fails with dependency errors”**
  - Update Rust:
    ```bash
    rustup update
    ```
  - Install `libpq-dev`:
    ```bash
    sudo apt-get install libpq-dev
    ```
  - Clear cache:
    ```bash
    cargo clean
    ```

- **“No logs appear”**
  - Verify `log4rs.yaml` and set `level: debug`.

## Getting Help

- Check logs in `log/app.log` or console.
- Visit [https://github.com/yourusername/sticker-recommendation-system](https://github.com/yourusername/sticker-recommendation-system) for issues.
- Ask in [r/rust](https://www.reddit.com/r/rust/) or [Rust Discord](https://discord.gg/rust-lang).

## Contributing

- Fork: [https://github.com/yourusername/sticker-recommendation-system](https://github.com/yourusername/sticker-recommendation-system).
- Submit pull requests or report bugs via GitHub issues.

## License

MIT License. Use, modify, and share freely!

## Next Steps

- Implement sticker recommendation endpoints using `interactions` and `sticker_metrics` tables.
- Explore [Actix Web docs](https://actix.rs/docs/) and [SQLx docs](https://docs.rs/sqlx).
- Integrate with a frontend using `FRONTEND_URL`.

Happy coding with the Sticker Recommendation System! 🚀
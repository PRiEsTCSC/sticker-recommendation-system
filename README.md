# Welcome to sticker-proj!

This beginner-friendly Rust web application template uses the **Actix web framework** and **postgres** to help you quickly build APIs or web applications. Whether you’re new to Rust or an experienced developer, this template makes setup easy and lets you focus on building your project.

## Why Use This Template?

- **Fast Web Server**: Powered by Actix web with built-in logging and rate limiting.
- **Database Ready**: Connects to postgres using SQLx for reliable database operations.
- **CORS Support**: Optional Cross-Origin Resource Sharing for browser-based APIs.
- **Docker Support**: Optional containerization for easy deployment.
- **Quick Testing**: Includes a `/health` endpoint to check if your server is running.

## What You’ll Need

Before you begin, ensure you have the following:

- **Rust**: The latest stable version. Install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  After installation, run `rustc --version` to confirm.
- **cargo-generate**: A tool to generate your project from this template. Install it with:
  ```sh
  cargo install cargo-generate
  ```


- **PostgreSQL**: A running PostgreSQL server. Download from [postgresql.org](https://www.postgresql.org/download/).




## Step-by-Step Setup

Follow these simple steps to get your project up and running. We’ll guide you every step of the way!

### 1. Create Your Project

1. Run the following command to generate your project:
   ```sh
   cargo generate --git https://github.com/PRiEsTCSC/rust_webserver_template.git
   ```
2. Answer the prompts:
   - **Project name**: Enter a name for your project (e.g., `myapp`). This will be your project folder and Rust package name.
   - **Description**: Provide a short project description (e.g., “My cool API”).
   - **Author name**: Enter your name (e.g., “Jane Doe”).
   - **Database**: Choose `postgres`, `mysql`, or `sqlite`.
   - **Database name**: Choose a name for your database (e.g., `mydb`).
   - **Generate .env file**: Choose whether to create a `.env` file with default settings.
   - **Logging**: Decide if you want logging support.
   - **CORS**: Decide if you need CORS for browser-based APIs.
   - **Port**: Specify the server port (e.g., `8080`).
   - **Docker**: Choose whether to include Docker configuration.

   This creates a new folder (e.g., `myapp`) containing all project files.

### 2. Set Up Your Database


- **Install PostgreSQL**: Follow the instructions at [postgresql.org](https://www.postgresql.org/download/) for your operating system.
- **Create a Database**: Create a database named `sticker-proj`:
  ```sh
  psql -U postgres -c "CREATE DATABASE sticker-proj;"
  ```
- **Note**: Save your database username, password, and port (default: `5432`).





### 3. Configure Your Environment


- Create a `.env` file in your project folder:
  ```sh
  HOST=127.0.0.1
  PORT=8080
  
  DATABASE_URL=postgres://<username>:<password>@localhost:5432/sticker-proj
  
  ```
- Replace `<username>` and `<password>` with your database credentials. For SQLite, no credentials are needed.


### 4. Install Dependencies

Navigate to your project folder:
```sh
cd sticker-proj
```
Build the project to download and compile dependencies:
```sh
cargo build
```
This may take a few minutes the first time.

### 5. Run Your Application

Start the web server:
```sh
cargo run
```
- The server will start at `http://127.0.0.1:8080`.
- To verify it’s working, open a new terminal and run:
  ```sh
  curl http://127.0.0.1:8080/health
  ```
- Expected output: `Server is healthy`.



## What’s Included in Your Project?

| File/Folder             | Purpose                                                                 |
|-------------------------|-------------------------------------------------------------------------|
| `src/main.rs`           | The entry point, setting up logging and starting the server.             |
| `src/init.rs`           | Configures the Actix web server with logging, rate limiting, and CORS.   |
| `src/configs/env_load.rs` | Loads settings (`HOST`, `PORT`, `DATABASE_URL`) from the `.env` file.  |
| `src/models/database.rs` | Manages the connection pool to your postgres database using SQLx.   |
| `src/routes/health.rs`  | Defines the `/health` endpoint to check server status.                   |
| `log4rs.yaml`           | Configures logging output (e.g., to the console or a file).              |



## Customizing Your Project

### Adding New Endpoints

1. Create a new file in `src/routes/` (e.g., `example.rs`):
   ```rust
   use actix_web::{get, HttpResponse, Responder};

   #[get("/example")]
   async fn example() -> impl Responder {
       HttpResponse::Ok().body("Hello from the example endpoint!")
   }
   ```
2. Update `src/routes/mod.rs`:
   ```rust
   pub mod health;
   pub mod example;
   ```
3. Add the endpoint to `src/init.rs` in the `init_routes` function:
   ```rust
   cfg.service(example::example);
   ```
4. Restart the server and test:
   ```sh
   curl http://127.0.0.1:8080/example
   ```

### Adding Database Queries

1. In `src/models/database.rs`, add a function to query your database:
   ```rust
   impl DatabaseConnection {
       pub async fn get_users(&self) -> Result<Vec<String>, sqlx::Error> {
           let rows = sqlx::query_scalar("SELECT name FROM users")
               .fetch_all(&self.pool)
               .await?;
           Ok(rows)
       }
   }
   ```
   **Note**: Ensure your database has a `users` table, or modify the query for your schema.
2. Create a route in `src/routes/users.rs`:
   ```rust
   use actix_web::{get, web, HttpResponse, Responder};
   use crate::models::database::DatabaseConnection;

   #[get("/users")]
   async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
       match db.get_users().await {
           Ok(users) => HttpResponse::Ok().json(users),
           Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
       }
   }
   ```
3. Register the route in `src/init.rs` and `src/routes/mod.rs`.
4. Test with:
   ```sh
   curl http://127.0.0.1:8080/users
   ```

### Changing Settings

- **Logging**: Modify `log4rs.yaml` to change verbosity or output to a file:
  ```yaml
  appenders:
    file:
      kind: file
      path: "app.log"
      encoder:
        pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
    stdout:
      kind: console
      encoder:
        pattern: "{d(%Y-%m-%d %H:%M:%S)} [{t}] {l} - {m}{n}"
  root:
    level: debug
    appenders:
      - stdout
      - file
  ```
- **Rate Limiting**: Adjust settings in `src/init.rs`:
  ```rust
  .seconds_per_request(60) // Allow one request per minute
  .burst_size(10) // Allow bursts of up to 10 requests
  ```

- **CORS**: Modify CORS settings in `src/init.rs` (e.g., restrict origins):
  ```rust
  actix_cors::Cors::default()
      .allowed_origin("https://example.com")
  ```


## Troubleshooting

- **“Database connection failed”**
  - **Check**: Ensure `DATABASE_URL` in `.env` is correct.
  - **PostgreSQL/MySQL**: Verify the database server is running and credentials are valid.
  - **SQLite**: Confirm the file path is accessible and writable.
  - **Test**: Use `psql` (PostgreSQL) or `mysql` (MySQL) to test connectivity.

- **“Port already in use”**
  - Change `PORT` in `.env` (e.g., `8081`).
  - Stop conflicting processes:
    ```sh
    killall cargo
    ```

- **“Build fails with dependency errors”**
  - Update Rust: `rustup update`.
  - Clear cache: `cargo clean`.
  - Install system libraries:
    - PostgreSQL: `sudo apt-get install libpq-dev` (Ubuntu).
    - MySQL: `sudo apt-get install libmysqlclient-dev`.
    - SQLite: `sudo apt-get install libsqlite3-dev`.

- **“No logs appear”**
  - Verify `log4rs.yaml` exists and set `level: debug`:
    ```yaml
    root:
      level: debug
    ```

- **“Template generation fails”**
  - Clear cache:
    ```sh
    rm -rf ~/.cargo/git/db/rust_webserver_template-*
    ```
  - Retry: `cargo generate --git https://github.com/PRiEsTCSC/rust_webserver_template.git`.

## Getting Help

- Check logs in the terminal or `app.log` (if configured).
- Visit [https://github.com/PRiEsTCSC/rust_webserver_template](https://github.com/PRiEsTCSC/rust_webserver_template) for issues or support.
- Ask in Rust communities: [r/rust](https://www.reddit.com/r/rust/) or [Rust Discord](https://discord.gg/rust-lang).

## Contributing

Want to improve this template?
- Fork it at [https://github.com/PRiEsTCSC/rust_webserver_template](https://github.com/PRiEsTCSC/rust_webserver_template).
- Submit pull requests or report bugs via GitHub issues.

## License

MIT License. Use, modify, and share freely!

## Next Steps

- Add a new endpoint (see “Adding New Endpoints”).
- Try database queries (see “Adding Database Queries”).
- Explore [Actix web docs](https://actix.rs/docs/) or [SQLx docs](https://docs.rs/sqlx).

Happy coding with sticker-proj! 🚀
# sticker-recommendation-system

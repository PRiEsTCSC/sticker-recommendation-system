# Sticker Finder Chrome Extension

Sticker Finder is a Chrome extension that lets you select text on `x.com`, right-click to find stickers (GIFs) based on the text’s emotion, and open up to three tabs with sticker URLs from GIPHY. The project uses a Rust backend (Actix Web) for API handling, a PostgreSQL database for storing interactions, and a FastAPI server to process text emotions and fetch stickers.

This guide is for absolute beginners and covers setting up the extension, backend, and database, as well as using and troubleshooting the application.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Project Structure](#project-structure)
- [Setup Instructions](#setup-instructions)
  - [1. Set Up the Environment](#1-set-up-the-environment)
  - [2. Set Up the PostgreSQL Database](#2-set-up-the-postgresql-database)
  - [3. Set Up the FastAPI Server](#3-set-up-the-fastapi-server)
  - [4. Set Up the Rust Backend](#4-set-up-the-rust-backend)
  - [5. Set Up the Chrome Extension](#5-set-up-the-chrome-extension)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Before starting, install the following on your computer:

- **Google Chrome**: Latest version (download from [chrome.google.com](https://www.google.com/chrome/)).
- **Rust**: Install via [rustup](https://rustup.rs/) by running:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  After installation, verify with:
  ```bash
  rustc --version
  cargo --version
  ```
- **PostgreSQL**: Install version 15 or later ([PostgreSQL downloads](https://www.postgresql.org/download/)).
  - On macOS: `brew install postgresql`
  - On Ubuntu: `sudo apt update && sudo apt install postgresql postgresql-contrib`
  - On Windows: Use the installer from the PostgreSQL website.
- **Python 3.8+**: Install from [python.org](https://www.python.org/downloads/).
  - Verify with: `python3 --version`
- **pip**: Python package manager (usually included with Python).
  - Verify with: `pip3 --version`
- **Git**: For cloning the repository ([Git downloads](https://git-scm.com/downloads)).
  - Verify with: `git --version`
- **A GIPHY API Key**: Sign up at [developers.giphy.com](https://developers.giphy.com/) and create an app to get a free API key.
- **A Text Editor**: VS Code, Sublime Text, or any editor for editing code.
- **Terminal**: Use Terminal (macOS/Linux) or Command Prompt/PowerShell (Windows).

## Project Structure

The project consists of:
- **Chrome Extension** (`index.html`, `signup.html`, `index.js`, `background.js`, `manifest.json`, `icon.png`): Handles user login/signup and sticker finding via context menu on `x.com`.
- **Rust Backend** (`src/routes/recommend.rs`, `src/structs/database_structs.rs`, `src/database.rs`): Actix Web server for API endpoints (`/v1/sticker/find`, `/v1/auth/login/user`, `/v1/auth/register/user`).
- **FastAPI Server** (`sticker_api.py`): Preloads Python dependencies (`text2emotion`, `nltk`, GIPHY API) and serves `/detect_emotion` and `/search_stickers` endpoints.
- **PostgreSQL Database**: Stores user interactions and metrics.
- **Redis** (optional): Caches sticker URLs for faster responses.

## Setup Instructions

### 1. Set Up the Environment

1. **Clone the Repository**:
   Create a project folder and clone the code:
   ```bash
   mkdir sticker-finder
   cd sticker-finder
   git clone <your-repository-url> .
   ```
   Replace `<your-repository-url>` with the actual Git repository URL (or copy files manually if not using Git).

2. **Create a `.env` File**:
   In the project root, create a file named `.env`:
   ```bash
   touch .env
   ```
   Add the following, replacing placeholders with your values:
   ```
   DATABASE_URL=postgresql://username:password@localhost:5432/sticker_finder
   GIPHY_API_KEY=your_giphy_api_key
   REDIS_URL=redis://localhost:6379
   ```
   - `username` and `password`: Your PostgreSQL credentials.
   - `your_giphy_api_key`: Your GIPHY API key from [developers.giphy.com](https://developers.giphy.com/).
   - `REDIS_URL`: Optional; install Redis if using caching (see below).

3. **Install Redis (Optional)**:
   If using Redis for caching:
   - On macOS: `brew install redis`
   - On Ubuntu: `sudo apt install redis-server`
   - On Windows: Use WSL or a Redis Docker container.
   - Start Redis: `redis-server`
   - Verify: `redis-cli ping` (should return `PONG`).

### 2. Set Up the PostgreSQL Database

1. **Start PostgreSQL**:
   - On macOS: `brew services start postgresql`
   - On Ubuntu: `sudo service postgresql start`
   - On Windows: Start via the PostgreSQL installer or Services app.
   - Verify: `psql --version`

2. **Create the Database**:
   Log in to PostgreSQL:
   ```bash
   psql -U your_username
   ```
   Create the database:
   ```sql
   CREATE DATABASE sticker_finder;
   \q
   ```

3. **Set Up Tables**:
   In the project root, create a file `schema.sql` with:
   ```sql
   CREATE TABLE IF NOT EXISTS users (
       id UUID PRIMARY KEY,
       username VARCHAR NOT NULL UNIQUE,
       password_hash VARCHAR NOT NULL,
       created_at TIMESTAMP NOT NULL
   );

   CREATE TABLE IF NOT EXISTS interactions (
       id UUID PRIMARY KEY,
       user_id UUID REFERENCES users(id),
       input_text TEXT NOT NULL,
       detected_emotion VARCHAR NOT NULL,
       sticker_url TEXT NOT NULL,
       created_at TIMESTAMP NOT NULL
   );

   CREATE TABLE IF NOT EXISTS sticker_metrics (
       id UUID PRIMARY KEY,
       user_id UUID REFERENCES users(id),
       sticker_url TEXT NOT NULL,
       usage_count INTEGER DEFAULT 1,
       last_used TIMESTAMP NOT NULL
   );
   ```
   Run the schema:
   ```bash
   psql -U your_username -d sticker_finder -f schema.sql
   ```

4. **Verify Database**:
   Connect to the database:
   ```bash
   psql -U your_username -d sticker_finder
   ```
   Check tables:
   ```sql
   \dt
   ```
   You should see `users`, `interactions`, and `sticker_metrics`.

### 3. Set Up the FastAPI Server

1. **Install Python Dependencies**:
   In the project root, install required packages:
   ```bash
   pip3 install fastapi uvicorn httpx python-dotenv text2emotion nltk
   ```

2. **Verify `sticker_api.py`**:
   Ensure `sticker_api.py` is in the project root with the content from previous responses (preloads NLTK data and handles `/detect_emotion`, `/search_stickers`).

3. **Start the FastAPI Server**:
   ```bash
   python3 sticker_api.py
   ```
   Check logs for:
   - `DEBUG: NLTK data loaded`
   - `DEBUG: Starting FastAPI server`
   The server runs on `http://localhost:8000`.

4. **Test FastAPI Endpoints**:
   Open a browser or use `curl` to test:
   ```bash
   curl -X POST http://localhost:8000/detect_emotion -H "Content-Type: application/json" -d '{"input_text": "I love coffee"}'
   ```
   Expected response: `{"detected_emotion": "happy coffee"}`
   ```bash
   curl -X POST http://localhost:8000/search_stickers -H "Content-Type: application/json" -d '{"q": "happy coffee", "rating": "g"}'
   ```
   Expected response: Array with up to three sticker objects (e.g., `[{"url": "https://media.giphy.com/..."}, ...]`).

### 4. Set Up the Rust Backend

1. **Install Rust Dependencies**:
   In the project root, ensure `Cargo.toml` includes:
   ```toml
   [package]
   name = "sticker-proj"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   actix-web = "4"
   sqlx = { version = "0.7", features = ["runtime-actix-native-tls", "postgres", "uuid", "chrono"] }
   redis = { version = "0.25", features = ["tokio-comp"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   uuid = { version = "1.10", features = ["v4", "serde"] }
   chrono = { version = "0.4", features = ["serde"] }
   log = "0.4"
   env_logger = "0.11"
   reqwest = { version = "0.12", features = ["json"] }
   ```
   Run:
   ```bash
   cargo build
   ```

2. **Verify `recommend.rs`**:
   Ensure `src/routes/recommend.rs` matches the latest version (endpoint `/v1/sticker/find`, returns three URLs, uses FastAPI endpoints).

3. **Start the Rust Server**:
   ```bash
   cargo run
   ```
   Check logs for:
   - Server starting on `http://127.0.0.1:8080`.
   - No errors like `Failed to save interaction`.

4. **Test API Endpoints**:
   Test the sticker endpoint (requires a valid `user_token`):
   ```bash
   curl -X POST http://127.0.0.1:8080/v1/sticker/find -H "Content-Type: application/json" -H "Authorization: Bearer your_token" -d '{"input_text": "I love coffee", "username": "nobleman"}'
   ```
   Expected response:
   ```json
   {
     "detected_emotion": "happy coffee",
     "sticker_urls": [
       "https://media.giphy.com/...",
       "https://media.giphy.com/...",
       "https://media.giphy.com/..."
     ]
   }
   ```

### 5. Set Up the Chrome Extension

1. **Verify Extension Files**:
   Ensure the following files are in the project root:
   - `index.html`: Login popup.
   - `signup.html`: Signup popup.
   - `index.js`: Handles login/signup form submissions.
   - `background.js`: Handles context menu and sticker fetching.
   - `manifest.json`: Extension configuration.
   - `icon.png`: Extension icon (48x48 pixels).

   Update `background.js` to fix the issue of only one tab opening:
   ```javascript
   const API_BASE_URL = "http://127.0.0.1:8080";

   chrome.runtime.onInstalled.addListener((details) => {
     if (details.reason === "install" || details.reason === "update") {
       createContextMenuIfLoggedIn();
     }
   });

   function createContextMenu() {
     chrome.contextMenus.create({
       id: "find-sticker",
       title: "Find Sticker",
       contexts: ["selection"],
       documentUrlPatterns: ["https://x.com/*"]
     }, () => {
       if (chrome.runtime.lastError) {
         console.error("Error creating context menu:", chrome.runtime.lastError);
       } else {
         console.log("Context menu created successfullyrefer: 
         console.log("Context menu created successfully");
       }
     });
   }

   function createContextMenuIfLoggedIn() {
     chrome.storage.local.get(["user_token"], (result) => {
       if (result.user_token) {
         createContextMenu();
       } else {
         console.log("No user_token found, skipping context menu creation");
       }
     });
   }

   function openTabWithDelay(url, index, callback) {
     setTimeout(() => {
       console.log(`Attempting to open tab ${index + 1} with URL: ${url}`);
       chrome.tabs.create({ url }, (tab) => {
         if (chrome.runtime.lastError) {
           console.error(`Failed to open tab ${index + 1}:`, chrome.runtime.lastError);
         } else {
           console.log(`Opened tab ${index + 1} with ID: ${tab.id} for URL: ${url}`);
         }
         callback();
       });
     }, index * 500);
   }

   chrome.contextMenus.onClicked.addListener((info, tab) => {
     if (info.menuItemId === "find-sticker" && info.selectionText) {
       console.log("Context menu clicked with text:", info.selectionText);
       chrome.storage.local.get(["user_token", "username"], (result) => {
         const { user_token: token, username } = result;

         if (!token || !username) {
           console.log("Missing token or username, showing notification");
           chrome.notifications.create({
             type: "basic",
             iconUrl: "icon.png",
             title: "Sticker Finder",
             message: "Please log in via the extension popup."
           });
           return;
         }

         console.log("Fetching stickers for user:", username);
         fetch(`${API_BASE_URL}/v1/sticker/find`, {
           method: "POST",
           headers: {
             "Content-Type": "application/json",
             "Authorization": `Bearer ${token}`
           },
           body: JSON.stringify({ input_text: info.selectionText, username })
         })
           .then((res) => {
             if (!res.ok) {
               throw new Error(`HTTP error! status: ${res.status}`);
             }
             return res.json();
           })
           .then((data) => {
             console.log("Extension received response:", data);
             if (data.sticker_urls && Array.isArray(data.sticker_urls) && data.sticker_urls.length > 0) {
               let tabCount = 0;
               const openNextTab = () => {
                 if (tabCount < data.sticker_urls.length) {
                   const url = data.sticker_urls[tabCount];
                   if (url && typeof url === "string" && url.startsWith("https://")) {
                     openTabWithDelay(url, tabCount, () => {
                       tabCount++;
                       openNextTab();
                     });
                   } else {
                     console.warn(`Invalid or missing URL at index ${tabCount}:`, url);
                     tabCount++;
                     openNextTab();
                   }
                 }
               };
               openNextTab();
             } else {
               console.log("No valid sticker URLs in response");
               chrome.notifications.create({
                 type: "basic",
                 iconUrl: "icon.png",
                 title: "Sticker Finder",
                 message: "No stickers found."
               });
             }
           })
           .catch((err) => {
             console.error("Extension fetch error:", err);
             chrome.notifications.create({
               type: "basic",
               iconUrl: "icon.png",
               title: "Sticker Finder",
               message: `Error: ${err.message}`
             });
           });
       });
     }
   });

   chrome.storage.onChanged.addListener((changes, namespace) => {
     if (namespace === "local" && changes.user_token) {
       console.log("User token changed, updating context menu");
       chrome.contextMenus.removeAll(() => {
         if (changes.user_token.newValue) {
           createContextMenu();
         }
       });
     }
   });
   ```

2. **Load the Extension in Chrome**:
   - Open Chrome and go to `chrome://extensions/`.
   - Enable “Developer mode” (toggle in the top-right corner).
   - Click “Load unpacked” and select the project folder containing `manifest.json`, `index.html`, etc.
   - Verify the extension appears in Chrome with the name “Sticker Finder” and icon.

3. **Test Login/Signup**:
   - Click the extension icon in Chrome’s toolbar to open the popup (`index.html`).
   - Sign up with a username (e.g., “nobleman”) and password.
   - Check the popup shows “✅ Logged in as nobleman”.
   - Verify the context menu (“Find Sticker”) appears when right-clicking selected text on `https://x.com/*`.

### Usage

1. **Log In or Sign Up**:
   - Open the extension popup (click the icon in Chrome’s toolbar).
   - Enter a username and password to sign up or log in.
   - After successful login/signup, the popup shows “✅ Logged in as <username>”.

2. **Find Stickers**:
   - Visit `https://x.com/`.
   - Select text (e.g., “I love coffee”).
   - Right-click and choose “Find Sticker” from the context menu.
   - The extension sends the text to the Rust backend, which detects the emotion (e.g., “happy coffee”) and fetches up to three sticker URLs from the FastAPI server (via GIPHY).
   - Three new tabs should open, each displaying a sticker GIF.

3. **Log Out**:
   - Open the extension popup and click “Logout”.
   - The context menu disappears until you log in again.

## Troubleshooting

1. **Only One Tab Opens**:
   - **Symptom**: You see `Attempting to open tab 1 with URL: ...` in Chrome DevTools but no logs for tabs 2 and 3.
   - **Fix**:
     - Open Chrome DevTools: `chrome://extensions/` > “Inspect views: background page”.
     - Check Console for errors like `Failed to open tab 2: ...` or `Invalid or missing URL`.
     - Ensure `background.js` is the updated version with `openTabWithDelay` (above).
     - Verify Chrome allows popups: Settings > Privacy and security > Site Settings > Pop-ups and redirects > Allow for `https://x.com/*`.
     - Test in a new Chrome profile to rule out other extensions interfering.
     - Check logs for: `Opened tab 2 with ID: ...` and `Opened tab 3 with ID: ...`.

2. **No Tabs Open**:
   - **Symptom**: No tabs open, and no `Attempting to open tab` logs.
   - **Fix**:
     - Ensure you’re logged in (popup shows “✅ Logged in as <username>”).
     - Check DevTools Console for `Extension fetch error` or `HTTP error! status: 401`.
     - Verify the Rust server is running (`cargo run`) and FastAPI server is running (`python3 sticker_api.py`).
     - Test the API manually:
       ```bash
       curl -X POST http://127.0.0.1:8080/v1/sticker/find -H "Content-Type: application/json" -H "Authorization: Bearer your_token" -d '{"input_text": "I love coffee", "username": "nobleman"}'
       ```

3. **Fewer Than Three Stickers**:
   - **Symptom**: Response has fewer than three URLs in `sticker_urls`.
   - **Fix**:
     - Check FastAPI logs (`python3 sticker_api.py`):
       - Look for `DEBUG: Got 3 results from GIPHY` and `DEBUG: Returning 3 results`.
       - If errors like `ERROR: HTTP error: 403`, verify `GIPHY_API_KEY` in `.env`.
       - If `429 - Rate limit exceeded`, wait or get a new GIPHY API key.
     - Test FastAPI directly:
       ```bash
       curl -X POST http://localhost:8000/search_stickers -H "Content-Type: application/json" -d '{"q": "happy coffee", "rating": "g"}'
       ```

4. **Database Errors**:
   - **Symptom**: Rust logs show `Failed to save interaction: error returned from database`.
   - **Fix**:
     - Verify `database.rs` has the correct `save_interaction` (6 columns: `id, user_id, input_text, detected_emotion, sticker_url, created_at`).
     - Check the database schema:
       ```bash
       psql -U your_username -d sticker_finder -c "\d interactions"
       ```
       Ensure no extra columns (e.g., `admin_id`).
     - Run:
       ```bash
       psql -U your_username -d sticker_finder -c "ALTER TABLE interactions DROP COLUMN IF EXISTS admin_id;"
       ```

5. **No Context Menu**:
   - **Symptom**: “Find Sticker” doesn’t appear when right-clicking on `x.com`.
   - **Fix**:
     - Ensure you’re logged in (check popup).
     - Verify `manifest.json` has `contextMenus` permission and `documentUrlPatterns: ["https://x.com/*"]`.
     - Check DevTools Console for `Error creating context menu`.

6. **Login/Signup Fails**:
   - **Symptom**: Popup shows “❌ Invalid credentials” or “❌ Network error”.
   - **Fix**:
     - Ensure Rust server is running (`cargo run`).
     - Test login endpoint:
       ```bash
       curl -X POST http://127.0.0.1:8080/v1/auth/login/user -H "Content-Type: application/json" -d '{"username": "nobleman", "password": "your_password"}'
       ```
     - Check Rust logs for errors.

## Contributing

To contribute:
1. Fork the repository.
2. Create a branch: `git checkout -b feature/your-feature`.
3. Commit changes: `git commit -m "Add your feature"`.
4. Push to your fork: `git push origin feature/your-feature`.
5. Open a pull request.

Report issues or suggest features via the repository’s issue tracker.

## License

MIT License. See `LICENSE` file for details.
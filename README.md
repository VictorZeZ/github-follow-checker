# GitHub Follow Checker

A fast and lightweight CLI tool written in Rust to analyze GitHub followers and following relationships.

GitHub Follow Checker helps you discover:

- Users you follow but who do not follow you back.
- Users who follow you but you do not follow them.

Built with Rust and GitHub REST API.

---

## ✨ Features

- 🚀 Fast and lightweight command-line application
- 🔍 Compare followers and following lists
- 📚 Supports large follower/following lists with pagination
- 🔄 Check multiple GitHub accounts without restarting
- ❌ No GitHub authentication required
- 🦀 Built completely with Rust
- 🌐 Uses GitHub public API

---

## 📸 Example

```
GitHub username: octocat

Getting GitHub data...

================================
You follow, but they don't follow you back:
================================
- user_one
- user_two

================================
They follow you, but you don't follow them:
================================
- user_three

Done.

--------------------------------
Press Enter to check another account.
Type 'q' and press Enter to exit.
>
```

---

## 📦 Installation

### Download Release

Download the latest version from the GitHub Releases page.

For Windows:

```
github-follow-checker.exe
```

Extract the file and run it.

---

## 🛠 Build From Source

### Requirements

Before building the project, make sure you have:

- Rust
- Cargo

Install Rust:

https://www.rust-lang.org/tools/install

---

### Clone Repository

```bash
git clone https://github.com/YOUR_USERNAME/github-follow-checker.git
```

Navigate into the project:

```bash
cd github-follow-checker
```

Build the project:

```bash
cargo build --release
```

Run:

```bash
cargo run --release
```

The compiled executable will be available in:

```
target/release/
```

---

## ⚙️ How It Works

The application uses GitHub REST API endpoints:

```
GET /users/{username}/followers
GET /users/{username}/following
```

It retrieves both lists and compares them.

### Example

Followers:

```
Alice
Bob
Charlie
```

Following:

```
Alice
David
```

Result:

You follow but they don't follow you back:

```
David
```

They follow you but you don't follow them:

```
Bob
Charlie
```

---

## 🌐 API Limitations

This application uses GitHub's public API.

For public profiles:

- No authentication is required.
- GitHub applies rate limits to API requests.
- Normal usage is within the allowed limits.

---

## 🧰 Tech Stack

Built with:

- Rust 🦀
- Tokio
- Reqwest
- Serde
- GitHub REST API

---

## 📁 Project Structure

```
github-follow-checker
│
├── src
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 🚀 Future Improvements

Planned improvements:

- [ ] GitHub authentication support
- [ ] Export results to JSON format
- [ ] Export results to CSV format
- [ ] Interactive terminal UI
- [ ] Cross-platform binaries
- [ ] Automated GitHub Releases with CI/CD
- [ ] Better error handling and logging

---

## 📄 License

This project is licensed under the MIT License.

You are free to use, modify, and distribute this project.

---

## 👨‍💻 Author

Created by **YOUR_NAME**

GitHub:

https://github.com/YOUR_USERNAME

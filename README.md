# Multi-Database CSV Importer

## 🚀 Overview

A robust, flexible Rust-based CSV importer that supports multiple databases, real-time file watching, data validation, and notification systems.

## ✨ Features

- **Multi-Database Support**
  - PostgreSQL
  - MySQL
  - MariaDB
  - SQLite (Planned)

- **Advanced CSV Processing**
  - Real-time directory watching
  - Automatic CSV file detection
  - Data validation
  - Duplicate detection
  - Batch insertions

- **Flexible Notifications**
  - Telegram
  - Email (Planned)
  - Slack (Planned)
  - Custom notification channels

## 🛠 Installation

### Prerequisites

- Rust (latest stable version)
- Cargo
- Database servers (PostgreSQL, MySQL)

### Setup

1. Clone the repository
```bash
git clone https://github.com/irfnrdh/csv-import-db.git
cd csv-import-db
```

2. Install dependencies
```bash
cargo build
```

## 📝 Configuration

Create a `config.toml` file with your database and notification settings:

```toml
[database]
types = ["postgresql", "mysql"]
host = "localhost"
port = 5432
username = "your_username"
password = "your_password"

[notification]
telegram_token = "your_telegram_bot_token"
telegram_chat_id = "your_chat_id"
```

## 🚀 Usage

```bash
cargo run --release
```

## 📂 Project Structure

```
multi-db-csv-importer/
│
├── src/
│   ├── main.rs             # Main application entry point
│   ├── config/             # Configuration handling
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   └── notification.rs
│   │
│   ├── importers/          # CSV import logic
│   │   ├── mod.rs
│   │   ├── csv_processor.rs
│   │   └── validator.rs
│   │
│   ├── database/           # Database connectors
│   │   ├── mod.rs
│   │   ├── mysql.rs
│   │   ├── postgres.rs
│   │   └── sqlite.rs
│   │
│   └── notifications/      # Notification systems
│       ├── mod.rs
│       ├── telegram.rs
│       └── email.rs
│
├── tests/                  # Unit and integration tests
│   ├── database_tests.rs
│   └── importer_tests.rs
│
├── Cargo.toml              # Project dependencies
├── README.md               # Project documentation
├── LICENSE                 # Project license
└── .gitignore              # Git ignore file
```

## 🧪 Testing

Run tests with:
```bash
cargo test
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

## 📧 Contact

Irfannur Diah - irfnrdh@gmail.com

Project Link: [https://github.com/irfnrdh/csv-import-db](https://github.com/irfnrdh/csv-import-db)

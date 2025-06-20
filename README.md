# 🎬 Cinedex

Cinedex is a powerful and simple-to-use Discord bot that lets you search for detailed information about movies and TV shows using the [OMDb API](http://www.omdbapi.com/). Written in [Rust](https://www.rust-lang.org/) using the [Poise](https://github.com/serenity-rs/poise) framework on top of the [Serenity](https://github.com/serenity-rs/serenity) Discord API

## ✨ Features

- 🎥 Search for any movie, series, or episode by title
- 📆 View release year, genre, rating, runtime, directors, and more!
- 🌟 See IMDb ratings and metascores
- 🖼️ Get movie posters right in the embed (requires OMDb Patreon)

## 🔧 Setup

### Prerequisites

- Rust (edition 2021 or newer)
- A Discord bot token
- An OMDb API key ([get it here](http://www.omdbapi.com/apikey.aspx))

### Clone and Build

```bash
git clone https://github.com/yourusername/cinedex.git
cd cinedex
cargo build --release
```

### Environment Variables

Create a `.env` file in the project root

```env
discord_token=your_discord_token
omdb_api_key=your_omdb_api_key
```

### Run

```bash
cargo run --release
```

## 🧪 Usage

Once Cinedex is added to your server, you can use the following commands:

`/imdb <title>`

Example:
`/imdb <The Matrix>`

The bot will respond will respond with an embed showing details about the movie.

## License

This project is licensed under the [MIT License](LICENSE)

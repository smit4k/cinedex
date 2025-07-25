use poise::{serenity_prelude as serenity};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serde::Deserialize;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[derive(Debug, Deserialize)]
struct OmdbMovie {
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Year")]
    year: Option<String>,
    #[serde(rename = "Rated")]
    rated: Option<String>,
    #[serde(rename = "Released")]
    released: Option<String>,
    #[serde(rename = "Runtime")]
    runtime: Option<String>,
    #[serde(rename = "Genre")]
    genre: Option<String>,
    #[serde(rename = "Director")]
    director: Option<String>,
    #[serde(rename = "Actors")]
    actors: Option<String>,
    #[serde(rename = "Plot")]
    plot: Option<String>,
    #[serde(rename = "Poster")]
    poster: Option<String>,
    #[serde(rename = "imdbRating")]
    imdb_rating: Option<String>,
    #[serde(rename = "imdbID")]
    imdb_id: Option<String>,
    #[serde(rename = "BoxOffice")]
    box_office: Option<String>,
    #[serde(rename = "Response")]
    response: String,
    #[serde(rename = "Error")]
    error: Option<String>,
}

async fn fetch_movie_data_title(title: &str) -> Result<OmdbMovie, reqwest::Error> {
    let omdb_api_key = std::env::var("omdb_api_key").expect("Missing omdb_api_key!");
    let url = format!(
        "http://www.omdbapi.com/?apikey={}&t={}&plot=short",
        omdb_api_key, 
        title
    );

    let response = reqwest::get(&url).await?;
    response.json::<OmdbMovie>().await
}

async fn fetch_movie_data_id(id: &str) -> Result<OmdbMovie, reqwest::Error> {
    let omdb_api_key = std::env::var("omdb_api_key").expect("Missing omdb_api_key!");
    let url = format!(
        "https://www.omdbapi.com/?apikey={}&i={}",
        omdb_api_key,
        id,
    );

    let response = reqwest::get(&url).await?;
    response.json::<OmdbMovie>().await
}

/// Look up a movie
#[poise::command(slash_command, prefix_command)]
pub async fn imdb(
    ctx: Context<'_>,
    #[description = "Movie title"] title: Option<String>,
    #[description = "IMDb ID"] id: Option<String>,
) -> Result<(), Error> {
    
    let movie_result = if let Some(title) = title {
        fetch_movie_data_title(&title).await
    } else if let Some(id) = id {
        fetch_movie_data_id(&id).await
    } else {
        let embed = CreateEmbed::default()
            .title("❌ Error")
            .description("Please provide either a movie title or IMDb ID.");
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    };


    let movie = match movie_result {
        Ok(movie) => movie,
        Err(err) => {
            let embed = CreateEmbed::default()
                .title("❌ Error fetching data")
                .description(format!("Failed to fetch data from OMDb: {}", err));
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    if movie.response == "False" {
        let embed = CreateEmbed::default()
            .title("Movie not found")
            .description(format!(
                "Error: {}",
                movie.error.unwrap_or_else(|| "Unknown Error".to_string())
            ));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let embed = CreateEmbed::default()
        .title(movie.title.as_deref().unwrap_or("Unknown Title"))
        .description(movie.plot.as_deref().unwrap_or("No plot available."))
        .field("Released", movie.released.as_deref().unwrap_or("N/A"), true)
        .field("Rated", movie.rated.as_deref().unwrap_or("N/A"), true)
        .field("IMDb Rating", movie.imdb_rating.as_deref().unwrap_or("N/A"), true)
        .field("Runtime", movie.runtime.as_deref().unwrap_or("N/A"), true)
        .field("Genre", movie.genre.as_deref().unwrap_or("N/A"), true)
        .field("Director(s)", movie.director.as_deref().unwrap_or("N/A"), true)
        .field("Box Office", movie.box_office.as_deref().unwrap_or("N/A"), true)
        .field("Actors", movie.actors.as_deref().unwrap_or("N/A"), false)
        .footer(CreateEmbedFooter::new(format!(
            "IMDb ID: {}",
            movie.imdb_id.as_deref().unwrap_or("N/A")
        )))
        .color(0xDEB522);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

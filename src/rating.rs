use poise::{serenity_prelude as serenity};
use serenity::builder::{CreateEmbed};
use serde::Deserialize;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[derive(Debug, Deserialize)]
struct OmdbRating {
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Deserialize)]
struct OmdbRatings {
    #[serde(rename = "Ratings")]
    ratings: Option<Vec<OmdbRating>>,
    #[serde(rename = "Metascore")]
    metascore: Option<String>,
    #[serde(rename = "imdbRating")]
    imdb_rating: Option<String>,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Response")]
    response: String,
    #[serde(rename = "Error")]
    error: Option<String>,
}

async fn fetch_ratings(title: &str) -> Result<OmdbRatings, reqwest::Error> {
    let omdb_api_key = std::env::var("omdb_api_key").expect("Missing omdb_api_key!");
    let url = format!(
        "http://www.omdbapi.com/?apikey={}&t={}&plot=short",
        omdb_api_key, 
        title
    );

    let response = reqwest::get(&url).await?;
    response.json::<OmdbRatings>().await
}

/// Look up movie ratings and scores
#[poise::command(slash_command, prefix_command)]
pub async fn ratings(
    ctx: Context<'_>,
    #[description = "Movie title"] title: String,
) -> Result<(), Error> {
    let movie = fetch_ratings(&title).await?;

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

    let mut embed = CreateEmbed::default()
        .title(movie.title.as_deref().unwrap_or("Unknown Title"))
        .color(0xDEB522);

    if let Some(ratings) = &movie.ratings {
        for rating in ratings {
            embed = embed.field(&rating.source, &rating.value, false);
        }
    }

    if let Some(metascore) = &movie.metascore {
        embed = embed.field("Metascore", metascore, false);
    }

    if let Some(imdb_rating) = &movie.imdb_rating {
        embed = embed.field("IMDb Rating", imdb_rating, false);
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

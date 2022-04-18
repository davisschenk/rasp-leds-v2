#[macro_use]
extern crate rocket;
use rasp_leds_hal::*;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::sync::{Arc, Mutex};

type Result<T> = std::result::Result<T, Json<LedError>>;
type StateRunner = State<LedRunner>;

#[cfg(feature = "spotify")]
type SpotifyClient = State<Arc<Mutex<rspotify::AuthCodeSpotify>>>;

#[post("/on")]
async fn on(runner: &StateRunner) -> Result<()> {
    runner.on().await.map_err(Json)
}

#[post("/off")]
async fn off(runner: &StateRunner) -> Result<()> {
    runner.off().await.map_err(Json)
}

#[post("/power")]
async fn power(runner: &StateRunner) -> Result<()> {
    runner.power().await.map_err(Json)
}

#[post("/pattern", format = "json", data = "<pattern>")]
async fn pattern(pattern: Json<Pattern>, runner: &StateRunner) -> Result<()> {
    log::info!("Got pattern");
    runner.pattern(pattern.into_inner()).await.map_err(Json)
}

#[get("/history")]
async fn history(runner: &StateRunner) -> Result<Json<HistoryList>> {
    runner.history().await.map(Json).map_err(Json)
}

#[get("/info")]
async fn info(runner: &StateRunner) -> Result<Json<Info>> {
    runner.info().await.map(Json).map_err(Json)
}

#[cfg(feature = "spotify")]
#[get("/callback?<code>")]
async fn spotify_callback(client: &SpotifyClient, code: &str) -> Result<()> {
    use rspotify::clients::{BaseClient, OAuthClient};

    if let Ok(mut spotify) = client.lock() {
        spotify.request_token(code).unwrap();
        spotify.write_token_cache().unwrap();
    }

    Ok(())
}


#[derive(serde::Serialize)]
struct SpotifyResp {
    url: String
}

#[derive(Serialize)]
enum SpotifyError {
    NoAuth(SpotifyResp),
    LedError(LedError)
}

#[cfg(feature = "spotify")]
#[post("/spotify", data = "<pattern>", format = "json")]
async fn spotify_pattern(runner: &StateRunner, client: &SpotifyClient, pattern: Json<Pattern>) -> std::result::Result<(), Json<SpotifyError>> {
    use rspotify::clients::BaseClient;


    let spotify = client.lock().unwrap().clone();

    if (*spotify.get_token().lock().unwrap()).is_none() {
        return Err(Json(SpotifyError::NoAuth( SpotifyResp { url: spotify.get_authorize_url(false).unwrap()})))
    }

    let mut pattern = pattern.into_inner();
    pattern.set_client(spotify);
    runner.pattern(pattern).await.map_err(|x| Json(SpotifyError::LedError(x)))
}

#[cfg(feature = "spotify")]
fn get_spotify() -> Arc<Mutex<rspotify::AuthCodeSpotify>> {
    use rspotify::{
        clients::{BaseClient, OAuthClient},
        scopes, AuthCodeSpotify, Config, Credentials, OAuth,
    };

    let creds = Credentials::from_env().unwrap();
    let scopes = scopes!(
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "user-modify-playback-state"
    );

    let oauth = OAuth::from_env(scopes).unwrap();
    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Config::default()
    };

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    match spotify.read_token_cache(true) {
        Ok(Some(new_token)) => {
            let expired = new_token.is_expired();

            // Load token into client regardless of whether it's expired o
            // not, since it will be refreshed later anyway.
            *spotify.get_token().lock().unwrap() = Some(new_token);

            if expired {
                // Ensure that we actually got a token from the refetch
                match spotify.refetch_token().unwrap() {
                    Some(refreshed_token) => {
                        log::info!("Successfully refreshed expired token from token cache");
                        *spotify.get_token().lock().unwrap() = Some(refreshed_token)
                    }
                    _ => log::error!("Failed to refresh token from cache"),
                }
            }
        }
        _ => {}
    }

    Arc::new(Mutex::new(spotify))
}

#[launch]
async fn rocket() -> _ {
    env_logger::init();

    #[cfg(feature = "simulate")]
    let runner = LedRunner::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut runner = LedRunner::new(300, 18, 255);

    rocket::build()
        .manage(runner)
        .manage(get_spotify())
        .mount("/", routes![spotify_callback])
        .mount("/api", routes![on, off, pattern, power, history, info, spotify_pattern])
}

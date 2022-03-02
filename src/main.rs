#[macro_use]
extern crate rocket;
use rasp_leds_hal::*;
use rocket::serde::json::Json;
use rocket::State;

type Result<T> = std::result::Result<T, Json<LedError>>;
type StateRunner = State<LedRunner>;

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
    runner.pattern(pattern.into_inner()).await.map_err(Json)
}

#[get("/history")]
async fn history(runner: &StateRunner) -> Result<Json<HistoryList>> {
    runner.history().await.map(Json).map_err(Json)
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
        .mount("/api", routes![on, off, pattern, power, history])
}

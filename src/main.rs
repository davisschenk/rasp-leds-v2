#[macro_use] extern crate rocket;
use rasp_leds_hal::*;
use std::sync::{Arc, Mutex};
use rocket::State;
use rocket::serde::json::Json;

type StateRunner = State<Arc<Mutex<LedRunner>>>;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[post("/on")]
fn on(runner: &StateRunner) -> &'static str {
    if let Ok(mut leds) = runner.lock() {
        leds.on();
    }

    "Turned on"
}


#[post("/off")]
fn off(runner: &StateRunner) -> &'static str {
    if let Ok(mut leds) = runner.lock() {
        leds.off();
    }

    "Turned off"
}

#[post("/power")]
fn power(runner: &StateRunner) -> &'static str {
    if let Ok(mut leds) = runner.lock() {
        leds.power();
    }

    "Toggled"
}

#[post("/pattern", format = "json", data = "<pattern>")]
fn pattern(pattern: Json<Pattern>, runner: &StateRunner) -> &'static str {
    if let Ok(mut leds) = runner.lock() {
        leds.run_pattern(pattern.into_inner());
    }

    "Changed Pattern"
}

#[get("/history")]
fn history(runner: &StateRunner) -> Json<Vec<rasp_leds_hal::State>> {
    if let Ok(mut leds) = runner.lock() {
        return Json(leds.get_history());
    }

    return Json(vec![])
}

#[launch]
fn rocket() -> _ {
    #[cfg(feature = "simulate")]
    let mut runner = LedRunner::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut runner = LedRunner::new(300, 18, 255);

    runner.start();

    let managed_runner = Arc::new(Mutex::new(runner));

    rocket::build()
        .manage(managed_runner)
        .mount("/", routes![index])
        .mount("/api", routes![on, off, pattern, power, history])
}

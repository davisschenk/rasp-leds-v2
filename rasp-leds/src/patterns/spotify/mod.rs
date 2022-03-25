pub mod playing_color;
pub use playing_color::*;

// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

// #[enum_dispatch]
// pub trait SpotifyPattern: RunnablePattern {
//     fn set_client(&mut self, client: AuthCodeSpotify);
// }

// #[enum_dispatch(SpotifyPattern)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "snake_case", tag = "pattern"))]
// pub enum SpotifyPatterns {
//     PlayingColor(PlayingColor),
// }

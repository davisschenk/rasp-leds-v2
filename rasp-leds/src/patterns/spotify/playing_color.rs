use super::super::RunnablePattern;
use crate::controller::{Controller, LedController};
use crate::error::{Result, LedError};
use crate::Color;
use rspotify::model::{CurrentPlaybackContext, Device, FullTrack};
use rspotify::{
    model::{PlayableItem, RepeatState, TrackId},
    prelude::PlayableId,
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayingColor {
    pub tick_rate: u64,

    #[serde(skip)]
    spotify: Option<AuthCodeSpotify>,

    #[serde(skip)]
    playing_currently: Option<TrackId>,

    color: Color,
}

impl RunnablePattern for PlayingColor {
    fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    fn tick_cycle(&self) -> Option<u64> {
        None
    }

    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()> {
        if let Some(spotify) = &self.spotify {
            if !self.currently_playing()? {
                self.color = Color::rand();
            }
        }

        for i in controller.get_data().iter_mut() {
            *i = self.color;
        }

        controller.update()
    }

    fn init(&mut self, _controller: &mut Controller) -> Result<()> {
        if self.spotify.is_none() {
            return Err(LedError::PatternError);
        }

        Ok(())
    }

    fn start_tick(&mut self, raw_tick: u64, leds: &mut Controller) -> Result<()> {
        match self.tick_cycle() {
            Some(cycle) => self.tick(raw_tick % cycle, leds),
            None => self.tick(raw_tick, leds),
        }
    }
    fn set_client(&mut self, client: AuthCodeSpotify) {
        self.spotify = Some(client);
    }
}

impl PlayingColor {
    fn currently_playing(&mut self) -> Result<bool> {
        if let Some(spotify) = &self.spotify {
            let track = spotify.current_playback(None, None::<Vec<_>>)?;
            if let Some(CurrentPlaybackContext {
                item: Some(PlayableItem::Track(FullTrack { id: Some(id), .. })),
                ..
            }) = track
            {
                if self.playing_currently.is_some()
                    && self.playing_currently.as_ref().unwrap() == &id
                {
                    return Ok(true);
                }

                self.playing_currently = Some(id);
            }
        }

        Ok(false)
    }

}

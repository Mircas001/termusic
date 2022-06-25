/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{PlayerMsg, PlayerTrait};
use crate::config::Termusic;
use anyhow::{anyhow, bail, Result};
use fragile::Fragile;
use gst::ClockTime;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_player as gst_player;
use std::cmp;
// use std::rc::Rc;
use glib::MainContext;
use std::sync::mpsc::Sender;

pub struct GStreamer {
    player: gst_player::Player,
    paused: bool,
    volume: i32,
    speed: f32,
    pub gapless: bool,
    tx: Sender<PlayerMsg>,
}

impl GStreamer {
    pub fn new(config: &Termusic, message_tx: Sender<PlayerMsg>) -> Self {
        gst::init().expect("Couldn't initialize Gstreamer");
        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let player = gst_player::Player::new(
            None,
            Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );

        let volume = config.volume;
        player.set_volume(f64::from(volume) / 100.0);
        let speed = config.speed;
        player.set_rate(speed.into());

        let (tx, rx) = MainContext::channel(glib::PRIORITY_DEFAULT);

        let tx_fragile = Fragile::new(tx);
        player.connect_end_of_stream(move |_| {
            eprintln!("ready to send eos");
            tx_fragile.get().send(PlayerMsg::Eos).unwrap();
        });

        let tx = message_tx.clone();
        rx.attach(None, move |_action| {
            tx.send(PlayerMsg::Eos).unwrap();
            glib::Continue(true)
        });
        Self {
            player,
            paused: false,
            volume,
            speed,
            gapless: true,
            tx: message_tx,
        }
    }
    pub fn skip_one(&mut self) {
        self.tx.send(PlayerMsg::Eos).unwrap();
    }
    pub fn enqueue_next(&mut self, next_track: &str) {
        self.player
            .set_uri(Some(&format!("file:///{}", next_track)));
    }
    pub fn play(&mut self) {
        self.player.stop();
        self.player.play();
    }
}

impl PlayerTrait for GStreamer {
    fn add_and_play(&mut self, song_str: &str) {
        self.player.set_uri(Some(&format!("file:///{}", song_str)));
        self.paused = false;

        self.play();
    }

    fn volume_up(&mut self) {
        self.volume = cmp::min(self.volume + 5, 100);
        self.player.set_volume(f64::from(self.volume) / 100.0);
    }

    fn volume_down(&mut self) {
        self.volume = cmp::max(self.volume - 5, 0);
        self.player.set_volume(f64::from(self.volume) / 100.0);
    }

    fn volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, mut volume: i32) {
        if volume > 100 {
            volume = 100;
        } else if volume < 0 {
            volume = 0;
        }
        self.volume = volume;
        self.player.set_volume(f64::from(volume) / 100.0);
    }

    fn pause(&mut self) {
        self.paused = true;
        self.player.pause();
    }

    fn resume(&mut self) {
        self.paused = false;
        self.player.play();
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    #[allow(clippy::cast_sign_loss)]
    fn seek(&mut self, secs: i64) -> Result<()> {
        if let Ok((_, time_pos, duration)) = self.get_progress() {
            let mut seek_pos = time_pos + secs;
            if seek_pos < 0 {
                seek_pos = 0;
            }

            if seek_pos.cmp(&duration) == std::cmp::Ordering::Greater {
                bail! {"exceed max length"};
            }
            self.player.seek(ClockTime::from_seconds(seek_pos as u64));
        }
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    fn get_progress(&mut self) -> Result<(f64, i64, i64)> {
        let time_pos = match self.player.position() {
            Some(t) => ClockTime::seconds(t).try_into().unwrap_or(0),
            None => 0_i64,
        };
        let duration = match self.player.duration() {
            Some(d) => ClockTime::seconds(d).try_into().unwrap_or(0),
            None => 0_i64,
        };
        let mut percent = (time_pos * 100)
            .checked_div(duration)
            .ok_or_else(|| anyhow!("divide error"))?;
        if percent > 100 {
            percent = 100;
        }
        Ok((percent as f64, time_pos, duration))
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
        self.player.set_rate(speed.into());
    }

    fn speed_up(&mut self) {
        let mut speed = self.speed + 0.1;
        if speed > 3.0 {
            speed = 3.0;
        }
        self.set_speed(speed);
    }

    fn speed_down(&mut self) {
        let mut speed = self.speed - 0.1;
        if speed < 0.1 {
            speed = 0.1;
        }
        self.set_speed(speed);
    }
    fn stop(&mut self) {
        self.player.stop();
    }
}

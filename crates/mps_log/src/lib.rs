// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use time::macros::format_description;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;

const FORMAT_PRETTY: &str = "pretty";
const FORMAT_COMPACT: &str = "compact";
const FORMAT_JSON: &str = "json";
const FORMAT_FULL: &str = "full";

#[derive(Debug, Clone)]
pub struct MpsLog {
    filter_level: String,
    with_ansi: bool,
    format: String,
    with_level: bool,
    with_target: bool,
    with_thread_ids: bool,
    with_thread_names: bool,
    with_source_location: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

impl Default for MpsLog {
    fn default() -> Self {
        MpsLog {
            filter_level: "info".to_owned(),
            with_ansi: true,
            format: FORMAT_PRETTY.to_owned(),
            with_level: true,
            with_target: true,
            with_thread_ids: true,
            with_thread_names: true,
            with_source_location: true,
        }
    }
}

impl MpsLog {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn filter_level(mut self, filter_level: &str) -> Self {
        self.filter_level = filter_level.to_owned();
        self
    }

    pub fn with_ansi(mut self, with_ansi: bool) -> Self {
        self.with_ansi = with_ansi;
        self
    }

    pub fn format(mut self, format: &str) -> Self {
        if format != FORMAT_PRETTY
            && format != FORMAT_COMPACT
            && format != FORMAT_JSON
            && format != FORMAT_FULL
        {
            panic!("Unknown format")
        }
        self.format = format.to_owned();
        self
    }

    pub fn with_level(mut self, with_level: bool) -> Self {
        self.with_level = with_level;
        self
    }

    pub fn with_target(mut self, with_target: bool) -> Self {
        self.with_target = with_target;
        self
    }

    pub fn with_thread_ids(mut self, with_thread_ids: bool) -> Self {
        self.with_thread_ids = with_thread_ids;
        self
    }

    pub fn with_thread_names(mut self, with_thread_names: bool) -> Self {
        self.with_thread_names = with_thread_names;
        self
    }

    pub fn with_source_location(mut self, with_source_location: bool) -> Self {
        self.with_source_location = with_source_location;
        self
    }

    pub fn init(self) -> Result<Self, Error> {
        // Local offset timezone init, and set time format.
        let offset = clia_local_offset::current_local_offset()
            .expect("Can not get local offset!");
        // println!("offset: {:?}", offset);
        let timer = OffsetTime::new(
            offset,
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        );

        // Tracing subscriber init.
        let s = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or(tracing_subscriber::EnvFilter::new(
                        &self.filter_level,
                    )),
            )
            .with_ansi(self.with_ansi);

        // Format switch.
        if self.format == FORMAT_PRETTY {
            let s = s
                .event_format(
                    fmt::format()
                        .pretty()
                        .with_level(self.with_level)
                        .with_target(self.with_target)
                        .with_thread_ids(self.with_thread_ids)
                        .with_thread_names(self.with_thread_names)
                        .with_source_location(self.with_source_location),
                )
                .with_timer(timer);
            s.with_writer(std::io::stdout).init();
        } else if self.format == FORMAT_COMPACT {
            let s = s
                .event_format(
                    fmt::format()
                        .compact()
                        .with_level(self.with_level)
                        .with_target(self.with_target)
                        .with_thread_ids(self.with_thread_ids)
                        .with_thread_names(self.with_thread_names)
                        .with_source_location(self.with_source_location),
                )
                .with_timer(timer);
            s.with_writer(std::io::stdout).init();
        } else if self.format == FORMAT_JSON {
            let s = s
                .event_format(
                    fmt::format()
                        .json()
                        .with_level(self.with_level)
                        .with_target(self.with_target)
                        .with_thread_ids(self.with_thread_ids)
                        .with_thread_names(self.with_thread_names)
                        .with_source_location(self.with_source_location),
                )
                .with_timer(timer);
            s.json().with_writer(std::io::stdout).init();
        } else if self.format == FORMAT_FULL {
            let s = s
                .event_format(
                    fmt::format()
                        .with_level(self.with_level)
                        .with_target(self.with_target)
                        .with_thread_ids(self.with_thread_ids)
                        .with_thread_names(self.with_thread_names)
                        .with_source_location(self.with_source_location),
                )
                .with_timer(timer);
            s.with_writer(std::io::stdout).init();
        }

        Ok(self)
    }
}

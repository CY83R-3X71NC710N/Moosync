// Moosync
// Copyright (C) 2024, 2025  Moosync <support@moosync.app>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(not(feature = "extensions"))]
use std::{
    fmt::Error as FmtError,
    num::{ParseFloatError, ParseIntError},
    string::FromUtf8Error,
};

#[cfg(all(
    not(feature = "extensions"),
    any(feature = "core", feature = "extensions-core")
))]
use std::io;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use std::time::SystemTimeError;

#[cfg(all(not(feature = "extensions"), feature = "core"))]
use fast_image_resize::ResizeError;

#[cfg(all(not(feature = "extensions"), feature = "core"))]
use google_youtube3::Error as YoutubeError;

#[cfg(all(not(feature = "extensions"), feature = "core"))]
use jsonschema::ValidationError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use rspotify::{model::IdError, ClientError};

#[cfg(all(not(feature = "extensions"), feature = "ui"))]
use serde_json::Value;

#[cfg(all(not(feature = "extensions"), feature = "ui"))]
use wasm_bindgen::JsValue;

#[cfg(all(not(feature = "extensions"), feature = "core"))]
use core::str;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use fast_image_resize::ImageBufferError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use hex::FromHexError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use image::ImageError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use keyring::Error as KeyringError;
#[cfg(all(
    not(feature = "extensions"),
    any(feature = "core", feature = "librespot")
))]
use librespot::core::Error as LibrespotError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use lofty::error::LoftyError;
#[cfg(all(not(feature = "extensions"), feature = "core"))]
use rusty_ytdl::VideoError;

#[cfg(not(feature = "extensions"))]
#[derive(Debug, thiserror::Error)]
pub enum MoosyncError {
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    Tauri(#[from] tauri::Error),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    Diesel(#[from] diesel::result::Error),
    #[cfg_attr(any(feature = "core", feature = "extensions-core"), error(transparent))]
    #[cfg(any(feature = "core", feature = "extensions-core"))]
    IO(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    Youtube(#[from] VideoError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    DotPaths(#[from] json_dotpath::Error),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    SystemTimeError(#[from] SystemTimeError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    ImageBufferError(#[from] ImageBufferError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    ImageError(#[from] ImageError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    DifferentTypesOfPixelsError(#[from] ResizeError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    LoftyError(#[from] LoftyError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    JWalkError(#[from] jwalk::Error),
    #[cfg_attr(any(feature = "core", feature = "librespot"), error(transparent))]
    #[cfg(any(feature = "core", feature = "librespot"))]
    Librespot(#[from] LibrespotError),
    #[error(transparent)]
    UTF8(#[from] FromUtf8Error),
    #[cfg_attr(
        any(feature = "core", feature = "librespot", feature = "extensions-core"),
        error(transparent)
    )]
    #[cfg(any(feature = "core", feature = "librespot", feature = "extensions-core"))]
    Reqwest(#[from] reqwest::Error),
    #[cfg_attr(any(feature = "core", feature = "librespot"), error(transparent))]
    #[cfg(any(feature = "core", feature = "librespot"))]
    ProtoBuf(#[from] protobuf::Error),
    #[error("{0}")]
    String(String),
    #[cfg_attr(any(feature = "core", feature = "extensions-core"), error(transparent))]
    #[cfg(any(feature = "core", feature = "extensions-core"))]
    ZipError(#[from] zip::result::ZipError),
    #[cfg_attr(any(feature = "core", feature = "extensions-core"), error(transparent))]
    #[cfg(any(feature = "core", feature = "extensions-core"))]
    FSExtraError(#[from] fs_extra::error::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    SpotifyError(#[from] ClientError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    SpotifyIdError(#[from] IdError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    YoutubeError(#[from] YoutubeError),
    #[cfg(feature = "core")]
    #[error("Transfer control to provider: {0}")]
    SwitchProviders(String),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    HexError(#[from] FromHexError),
    #[cfg_attr(feature = "core", error(transparent))]
    #[cfg(feature = "core")]
    KeyringError(#[from] KeyringError),
    #[cfg(feature = "core")]
    #[cfg_attr(feature = "core", error("JSON validation failed: {0}"))]
    JSONValidationError(String),
    #[error(transparent)]
    FmtError(#[from] FmtError),
    #[cfg(feature = "core")]
    #[error(transparent)]
    RodioError(#[from] rodio::StreamError),
    #[cfg(feature = "core")]
    #[error(transparent)]
    RodioDecoderError(#[from] rodio::decoder::DecoderError),
    #[cfg(feature = "core")]
    #[error(transparent)]
    RodioSeekError(#[from] rodio::source::SeekError),
    #[cfg(feature = "core")]
    #[error(transparent)]
    UTF8Error(#[from] str::Utf8Error),

    #[cfg(feature = "core")]
    #[error(transparent)]
    HLSError(#[from] hls_client::errors::HLSDecoderError),

    #[error("Invalidated cache")]
    InvalidatedCache,
}

#[cfg(all(not(feature = "extensions"), feature = "ui"))]
impl From<serde_wasm_bindgen::Error> for MoosyncError {
    #[tracing::instrument(level = "debug", skip(value))]
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        Self::String(value.to_string())
    }
}

#[cfg(all(not(feature = "extensions"), feature = "core"))]
impl<'a> From<ValidationError<'a>> for MoosyncError {
    #[tracing::instrument(level = "debug", skip(value))]
    fn from(value: ValidationError<'a>) -> Self {
        let mut res = String::new();
        res.push_str(value.to_string().as_str());
        res.push('\n');

        Self::JSONValidationError(res)
    }
}

#[cfg(all(not(feature = "extensions"), feature = "ui"))]
impl From<JsValue> for MoosyncError {
    #[tracing::instrument(level = "debug", skip(value))]
    fn from(value: JsValue) -> Self {
        let parsed: Value = serde_wasm_bindgen::from_value(value).unwrap();
        Self::String(format!("{}", parsed))
    }
}

impl From<&'static str> for MoosyncError {
    #[tracing::instrument(level = "debug", skip(value))]
    fn from(value: &'static str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for MoosyncError {
    #[tracing::instrument(level = "debug", skip(value))]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl serde::Serialize for MoosyncError {
    #[tracing::instrument(level = "debug", skip(self, serializer))]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg(feature = "extensions")]
#[derive(Debug, thiserror::Error)]
pub enum MoosyncError {
    #[error("{0}")]
    String(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, MoosyncError>;

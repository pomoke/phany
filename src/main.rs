use clap::Parser;
use cli::Args;
use iced::{
    font::{Family, Weight},
    Application, Settings,
};
use ui::ViewerUI;

pub mod cli;
pub mod components;
pub mod config;
pub mod db;
pub mod iop;
pub mod loader;
pub mod ui;

use iced_aw::BOOTSTRAP_FONT_BYTES;

fn main() {
    let arg = Args::parse();

    ViewerUI::run(Settings {
        flags: arg
            .file
            .map(|x| x.to_string_lossy().into_owned())
            .unwrap_or("".to_owned()),
        fonts: vec![BOOTSTRAP_FONT_BYTES.into()],
        default_font: iced::Font {
            family: Family::Name("Noto Sans"),
            weight: Weight::Normal,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}

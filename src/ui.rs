use std::path::Path;
use std::path::PathBuf;

use crate::components::image::ViewerUI;
use crate::components::viewer::Viewer;
use crate::iop::image::Image;
use iced::advanced::widget::Text;
use iced::advanced::Widget;
use iced::alignment;
use iced::event;
use iced::executor;
use iced::keyboard;
use iced::keyboard::key;
use iced::keyboard::Key;
use iced::theme;
use iced::widget::button;
use iced::widget::component;
use iced::widget::image::Handle;
use iced::widget::text::Shaping;
use iced::widget::Space;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::Application;
use iced::Command;
use iced::Event;
use iced::Padding;
use iced::Size;
use iced::Theme;
use iced::Vector;
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};
use iced_aw::{Bootstrap, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES};

#[derive(Default)]
pub struct MainUI {
    viewer: Option<image::Handle>,
    filename: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MainEvent {
    Ready(image::Handle),
    ZoomIn,
    ZoomOut,
    ZoomOriginal
}

impl Application for MainUI {
    type Message = MainEvent;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = String;

    fn new(file: Self::Flags) -> (Self, Command<Self::Message>) {
        let name = Path::new(&file);
        let filename = name.file_name().map_or("", |x| x.to_str().unwrap_or("?"));

        let s = Self {
            filename: Some(filename.to_owned()),
            ..Default::default()
        };
        (
            s,
            Command::perform(
                async {
                    let viewer = image::Handle::from_path(file);
                    viewer
                },
                |x| MainEvent::Ready(x),
            ),
        )
    }

    fn title(&self) -> String {
        self.filename
            .as_deref()
            .map(|x| format!("{} - phany", x))
            .unwrap_or("phany".to_owned())
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen_with(|e, s| match e {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                location,
                modifiers,
                text,
            }) => {
                if (key == Key::Character("0".into())) && modifiers.control() {
                    Some(MainEvent::ZoomOriginal)
                } else if (key == Key::Character("=".into())) && modifiers.control() {
                    Some(MainEvent::ZoomIn)
                } else if (key == Key::Character("-".into())) && modifiers.control() {
                    Some(MainEvent::ZoomOut)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            MainEvent::Ready(handle) => {
                self.viewer = Some(handle);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        if let Some(ref handle) = self.viewer {
            let viewer = ViewerUI::default().set_handle(handle.clone()).set_scale(1.);
            component(viewer)
        } else {
            container(
                text("Loading...")
                    .size(36)
                    .shaping(Shaping::Advanced)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .into()
        }
    }
}

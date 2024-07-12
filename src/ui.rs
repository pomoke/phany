use std::path::Path;
use std::path::PathBuf;

use crate::components::viewer::Viewer;
use iced::advanced::widget::Text;
use iced::advanced::Widget;
use iced::alignment;
use iced::executor;
use iced::keyboard;
use iced::keyboard::key;
use iced::theme;
use iced::widget::button;
use iced::widget::image::Handle;
use iced::widget::text::Shaping;
use iced::widget::Image;
use iced::widget::Space;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::Application;
use iced::Command;
use iced::Size;
use iced::Theme;
use iced::Vector;
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};
use iced_aw::{Bootstrap, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES};

#[derive(Default)]
pub struct ViewerUI {
    viewer: Option<image::Handle>,
    scale: f32,
    position: Vector,
    filename: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ViewerEvents {
    ZoomIn,
    ZoomOut,
    Scale(f32),
    Move(Vector),
    RotateCW,
    RotateCCW,
    Reset,
    Open,
    Save,
    SaveAs,
    Export,
    Fullscreen,
    About,
    Preferences,
    Exit,
    Ready(Handle),
}

impl Application for ViewerUI {
    type Message = ViewerEvents;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = String;

    fn new(file: Self::Flags) -> (Self, Command<Self::Message>) {
        let name = Path::new(&file);
        let filename = name.file_name().map_or("", |x| x.to_str().unwrap_or("?"));

        let mut s = Self {
            scale: 1.,
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
                |x| ViewerEvents::Ready(x),
            ),
        )
    }

    fn title(&self) -> String {
        self.filename
            .as_deref()
            .map(|x| format!("{} - phany", x))
            .unwrap_or("phany".to_owned())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            ViewerEvents::Ready(v) => {
                self.viewer = Some(v);
            }
            ViewerEvents::Move(v) => {
                self.position = v;
            }
            ViewerEvents::Scale(f) => {
                self.scale = f;
            }
            ViewerEvents::Reset => {
                self.scale = 1.;
                self.position = Vector::default();
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut window = column![];
        if let Some(v) = &self.viewer {
            let viewer = Viewer::new(v.clone(), self.scale, self.position);
            let viewer = viewer
                .width(Length::Fill)
                .height(Length::Fill)
                .min_scale(0.5)
                .max_scale(20.)
                .on_scale(|x| ViewerEvents::Scale(x))
                .on_move(|x| ViewerEvents::Move(x));
            window = window.push(viewer);
        } else {
            window = window.push(Space::new(Length::Fill, Length::Fill));
            window = window.push(
                text("Loading...")
                    .size(24)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill),
            );
            window = window.push(Space::new(Length::Fill, Length::Fill));
        }
        let mut toolbar = row![
            button(
                text(Bootstrap::Info.to_string())
                    .size(24)
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .padding(6)
            .style(theme::Button::Text),
            button(
                text(Bootstrap::ArrowClockwise.to_string())
                    .size(24)
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .padding(6)
            .style(theme::Button::Text),
            button(
                text(Bootstrap::ArrowCounterclockwise.to_string())
                    .size(24)
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .padding(6)
            .style(theme::Button::Text),
            button(
                text("reset")
                    .size(24)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .shaping(Shaping::Advanced)
            )
            .padding(6)
            .style(theme::Button::Text)
            .on_press(ViewerEvents::Reset),
            Space::new(Length::Fill, Length::Shrink),
            text(format!(
                "{},{} {}",
                self.position.x, self.position.y, self.scale
            ))
            .shaping(Shaping::Advanced)
            .size(24)
            .height(Length::Shrink)
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center),
        ];
        window = window.push(toolbar.padding(4));
        container(window)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}

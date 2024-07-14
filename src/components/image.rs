use std::path::Path;
use std::path::PathBuf;

use crate::components::viewer::Viewer;
use crate::ui::MainEvent;
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
use iced::widget::image::Handle;
use iced::widget::text::Shaping;
use iced::widget::Image;
use iced::widget::Space;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Component, Container, Slider};
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
pub struct ViewerUI {
    viewer: Option<image::Handle>,
    scale: Option<f32>,
    position: Option<Vector>,
    filename: Option<String>,
    //display_metadata: bool,
}

pub struct ViewerState {
    scale: f32,
    position: Vector,
    display_metadata: bool,
}

impl Default for ViewerState {
    fn default() -> Self {
        Self {
            scale: 1.,
            position: Vector::new(0., 0.),
            display_metadata: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ViewerEvent {
    ZoomIn,
    ZoomOut,
    ZoomOriginal,
    ZoomChange,
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
    Info,
    Exit,
    Ready(Handle),
}

#[derive(Debug, Clone)]
pub enum ViewerMessage {}

impl ViewerUI {
    pub fn set_handle(mut self,handle: Handle) -> Self {
        self.viewer = Some(handle);
        self
    }

    pub fn set_position(mut self, position: Vector) -> Self {
        self.position = Some(position);
        self
    }

    pub fn set_scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }
}

impl Component<MainEvent> for ViewerUI {
    /*
    fn new(file: Self::Flags) -> (Self, Command<Self::Message>) {
        let name = Path::new(&file);
        let filename = name.file_name().map_or("", |x| x.to_str().unwrap_or("?"));

        let s = Self {
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
                |x| ViewerMessage::Ready(x),
            ),
        )
    }
    */
    type Event = ViewerEvent;
    type State = ViewerState;


    fn update(&mut self, state: &mut ViewerState, event: ViewerEvent) -> Option<MainEvent> {
        match event {
            ViewerEvent::Ready(v) => {
                self.viewer = Some(v);
            }
            ViewerEvent::Move(v) => {
                state.position = v;
            }
            ViewerEvent::Scale(f) => {
                state.scale = f;
            }
            ViewerEvent::ZoomIn => {
                state.scale *= 1.1;
                state.scale = state.scale.clamp(0.1, 20.);
            }
            ViewerEvent::ZoomOut => {
                state.scale /= 1.1;
                state.scale = state.scale.clamp(0.1, 20.);
            }
            ViewerEvent::ZoomOriginal => {
                state.scale = 1.;
            }
            ViewerEvent::ZoomChange => {
                if state.scale < 1. {
                    state.scale = 1.;
                } else if state.scale < 2. {
                    state.scale = 2.;
                } else {
                    state.scale = 0.5;
                }
            }
            ViewerEvent::Reset => {
                state.scale = 1.;
                state.position = Vector::default();
            }
            ViewerEvent::Info => {
                state.display_metadata = !state.display_metadata;
            }
            _ => {}
        }
        None
    }

    fn view(
        &self,
        state: &Self::State,
    ) -> Element<
        Self::Event

    > {
        let mut window = column![];
        if let Some(v) = &self.viewer {
            let viewer = Viewer::new(v.clone());
            let viewer = viewer
                .width(Length::Fill)
                .height(Length::Fill)
                .min_scale(0.1)
                .max_scale(20.)
                .set_offset(state.position)
                .set_scale(state.scale)
                .on_scale(|x| ViewerEvent::Scale(x))
                .on_move(|x| ViewerEvent::Move(x))
                .on_middle(|| ViewerEvent::ZoomChange);
            let col = if state.display_metadata {
                let info_box = scrollable(
                    column![
                        text(self.filename.clone().unwrap_or("image".to_owned()))
                            .size(20)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .width(Length::Fill),
                        Space::new(Length::Shrink, Length::Fixed(8.)),
                        text("1").shaping(Shaping::Advanced),
                        text("1").shaping(Shaping::Advanced),
                        text("1").shaping(Shaping::Advanced),
                        text("1").shaping(Shaping::Advanced),
                    ]
                    .padding(Padding {
                        top: 4.,
                        bottom: 4.,
                        left: 20.,
                        right: 8.,
                    }),
                );
                container(row![
                    info_box.width(Length::Fixed(320.)),
                    viewer.width(Length::FillPortion(5))
                ])
            } else {
                container(column![viewer])
            };
            window = window.push(col);
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
        let toolbar = row![
            button(
                text(Bootstrap::InfoCircle.to_string())
                    .size(24)
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .padding(6)
            .style(theme::Button::Primary)
            .on_press(ViewerEvent::Info),
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
            .on_press(ViewerEvent::Reset),
            Space::new(Length::Fill, Length::Shrink),
            /*
            text(format!(
                "{},{} {}",
                self.position.x, self.position.y, self.scale
            ))
            .shaping(Shaping::Advanced)
            .size(24)
            .height(Length::Shrink)
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center),
            */
            button(
                text(format!("{:.0}%", state.scale * 100.))
                    .shaping(Shaping::Advanced)
                    .size(24)
                    .height(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .padding(6)
            .style(theme::Button::Text)
            .on_press(ViewerEvent::ZoomChange),
            button(
                text(Bootstrap::MenuUp)
                    .font(BOOTSTRAP_FONT)
                    .size(24)
                    .height(Length::Shrink)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .padding(6)
            .style(theme::Button::Text),
        ];
        window = window.push(toolbar.padding(4));
        container(window)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}

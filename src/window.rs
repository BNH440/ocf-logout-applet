use cosmic::app::Core;
use cosmic::iced::{
    platform_specific::shell::commands::popup::{destroy_popup, get_popup},
    window::Id,
    Limits,
};
use cosmic::iced_runtime::core::window;
use cosmic::{Action, Element, Task};

use cosmic::widget::{list_column, settings, text, button};
use std::time::Duration;

const ID: &str = "com.example.BasicApplet";

#[derive(Default)]
pub struct Window {
    core: Core,
    popup: Option<Id>,
}

#[derive(Clone, Debug)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    Logout,
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let window = Window {
            core,
            ..Default::default()
        };

        (window, Task::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Logout => {
                std::process::Command::new("loginctl")
                    .args(["terminate-session", "$XDG_SESSION_ID"])
                    .output()
                    .ok();
            }
            Message::TogglePopup => {
                return if let Some(popup_id) = self.popup.take() {
                    destroy_popup(popup_id)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );

                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);

                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(popup_id) => {
                if self.popup.as_ref() == Some(&popup_id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = text("Log Out")
            .size(30)
            .width(cosmic::iced::Length::Shrink);

        let button = button::custom(content)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::Logout)
            .padding([0, 12]);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        let content_list = list_column()
            .padding(5)
            .spacing(0)
            .add(settings::item(
                "Quota Status",
                "test",
            ));

        self.core.applet.popup_container(content_list).into()
    }
}

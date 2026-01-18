use iced::keyboard;
use iced::widget::{center, column, container, text};
use iced::window;
use iced::{Center, Element, Fill, Subscription, Task, Theme};

pub fn main() -> iced::Result {
    iced::application(Kiosk::new, Kiosk::update, Kiosk::view)
        .subscription(Kiosk::subscription)
        .theme(|_| Theme::Dark)
        .window(window::Settings {
            fullscreen: true,
            decorations: false,
            resizable: false,
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Default)]
struct Kiosk {
    frames: u64,
    last_frame: Option<iced::time::Instant>,
    fps: f32,
    mode: window::Mode,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Frame(iced::time::Instant),
    SetMode(window::Mode),
    Exit,
}

impl Kiosk {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                mode: window::Mode::Fullscreen,
                ..Self::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => {
                self.frames = self.frames.saturating_add(1);

                if let Some(last) = self.last_frame {
                    let dt = now.duration_since(last).as_secs_f32();
                    if dt > 0.0 {
                        // Exponential moving average to keep it stable
                        let instant_fps = 1.0 / dt;
                        self.fps = if self.fps == 0.0 {
                            instant_fps
                        } else {
                            self.fps * 0.9 + instant_fps * 0.1
                        };
                    }
                }

                self.last_frame = Some(now);
                Task::none()
            }
            Message::SetMode(mode) => {
                self.mode = mode;
                window::latest().and_then(move |id| window::set_mode(id, mode))
            }
            Message::Exit => iced::exit(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        let frames = window::frames().map(Message::Frame);

        let keys = keyboard::listen().filter_map(|event| match event {
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key),
                modifiers,
                ..
            } => match (key, modifiers) {
                (key::Named::Escape, _) => Some(Message::Exit),
                (key::Named::F11, _) => Some(Message::SetMode(match self.mode {
                    window::Mode::Fullscreen => window::Mode::Windowed,
                    _ => window::Mode::Fullscreen,
                })),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::SetMode(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::SetMode(window::Mode::Windowed))
                }
                _ => None,
            },
            _ => None,
        });

        Subscription::batch([frames, keys])
    }

    fn view(&self) -> Element<'_, Message> {
        let help = column![
            text("Kiosk (Wayland/X11) example").size(44),
            text("Esc: exit").size(22),
            text("F11: toggle fullscreen/windowed").size(22),
            text("Shift+↑: fullscreen   Shift+↓: windowed").size(22),
            text(format!("Frames: {}", self.frames)).size(22),
            text(format!("FPS (EMA): {:.1}", self.fps)).size(22),
        ]
        .spacing(12)
        .align_x(Center)
        .width(Fill);

        center(container(help).padding(32)).into()
    }
}


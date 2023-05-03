use iced::alignment::{Horizontal, Vertical};
use iced::widget::table::{Column, Length, Row, Table};
use iced::widget::{Button, Text, TextInput};
use iced::{Element, Padding, Sandbox, Settings};

pub fn main() -> iced::Result {
    TableDemo::run(Settings::default())
}

#[derive(Default)]
struct TableDemo {
    text_input_str: String,
    selected_rows: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    TextInputted(String),
    SelectedRows(Vec<bool>),
}

impl Sandbox for TableDemo {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Table - Iced".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectedRows(rows) => self.selected_rows = rows,
            Message::ButtonPressed => {
                dbg!("There is a button being pressed!");
            }
            Message::TextInputted(str) => {
                self.text_input_str = str;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let columns = vec![
            Column {
                width: Length::Fixed(100.0),
                alignment: (Horizontal::Center, Vertical::Center),
                cell_padding: Padding::from(10.0),
            },
            Column {
                width: Length::Fixed(500.0),
                alignment: (Horizontal::Left, Vertical::Top),
                cell_padding: Padding::from(10.0),
            },
            Column {
                width: Length::Fill,
                alignment: (Horizontal::Right, Vertical::Bottom),
                cell_padding: Padding::from(20.0),
            },
        ];

        let header = Row::new(
            vec![
                Some(Text::new("Index").into()),
                Some(Text::new("Sample Text").into()),
                Some(Text::new("What is the best Rust GUI library?").into()),
            ],
            80.0,
        );

        let rows = vec![
          Row::new(
                vec![
                    Some(Text::new(1.to_string()).into()),
                    Some(Text::new("Lorem ipsum dolor sit amet, \
                    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
                    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
                    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").into()),
                    Some(Text::new("ICED").into()),
                ],
                200.0,
            ),
            Row::new(
                vec![
                    None,
                    Some(Button::new("This is a button in a table!")
                        .on_press(Message::ButtonPressed)
                        .into()),
                    Some(TextInput::new("What belongs here?", &self.text_input_str)
                        .on_input(Message::TextInputted)
                        .into()),
                ],
                100.0,
            ),
        ];

        let table = Table::try_new(columns, rows)
            .unwrap()
            .fill_factor(1)
            .padding(Padding::new(50.0))
            .striped(true)
            .try_header(
                header,
                Some((Horizontal::Left, Vertical::Center)),
                Some(Padding::from(5.0)),
            )
            .unwrap();

        table.into()
    }
}

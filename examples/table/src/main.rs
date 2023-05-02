use iced::{Element, Padding, Sandbox, Settings};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::table::{Column, Length, Row, Table};
use iced::widget::Text;

pub fn main() -> iced::Result {
    TableDemo::run(Settings::default())
}

#[derive(Default)]
struct TableDemo {
    selected_rows: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Message {
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
        }
    }

    fn view(&self) -> Element<Self::Message> {

        let columns = vec![
            Column {
                width: Length::Fixed(100.0),
                alignment: (Horizontal::Center, Vertical::Center),
                cell_padding: Padding::from(5.0),
            },
            Column {
                width: Length::Fixed(500.0),
                alignment: (Horizontal::Left, Vertical::Top),
                cell_padding: Padding::from(5.0),
            },
            Column {
                width: Length::Fill,
                alignment: (Horizontal::Right, Vertical::Bottom),
                cell_padding: Padding::from(5.0),
            },
        ];

        let header = Row::new(
            vec![
                Text::new("Index"),
                Text::new("Sample Text"),
                Text::new("What is the best Rust GUI library?"),
            ],
            50.0
        );

        let mut rows = Vec::new();

        for index in 0..5 {
            rows.push(Row::new(
                vec![
                    Text::new(index.to_string()),
                    Text::new("Lorem ipsum dolor sit amet, \
                    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
                    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
                    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
                    Text::new("ICED"),
                ],
                200.0,
            ));
        }

        let table = Table::try_new(columns, rows).unwrap()
            .fill_factor(1)
            .padding(Padding::new(50.0))
            .striped(true)
            .try_header(header).unwrap();

        table.into()
    }
}
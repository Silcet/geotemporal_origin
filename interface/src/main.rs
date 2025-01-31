use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<Message> {
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        let counter = text(self.value);

        let interface = column![increment, counter, decrement];

        interface
    }
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        let mut counter = Counter { value: 0 };

        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);

        assert_eq!(counter.value, 1);
    }
}

use super::*;
use extern_timer::*;
use game::command::CommandSender;

#[derive(Clone)]
pub struct Guard {
    guard: Option<extern_timer::Guard>,
}

impl Guard {
    pub fn new(guard: Option<extern_timer::Guard>) -> Self {
        Self { guard }
    }
}

pub struct Timer {
    timer: MessageTimer<Command>,
    test_mode: bool,
    pub tags: HashMap<String, chrono::Duration>,
}

impl Timer {
    pub fn new(command_tx: Option<CommandSender>) -> Self {
        let command_tx = if command_tx.is_some() {
            command_tx.unwrap()
        } else {
            let (command_tx, _) = std::sync::mpsc::channel();
            command_tx
        };

        Self {
            timer: MessageTimer::new(command_tx.clone()),
            test_mode: false,
            tags: HashMap::new(),
        }
    }

    pub fn repeating<S: ToString>(
        &mut self,
        duration: chrono::Duration,
        command: Command,
        tag: S,
    ) -> Guard {
        self.tags.insert(tag.to_string(), duration);

        if self.test_mode {
            Guard::new(None)
        } else {
            let guard = self.timer.schedule_repeating(duration, command);
            Guard::new(Some(guard))
        }
    }

    pub fn set_test_mode(&mut self) {
        self.test_mode = true;
    }
}

use super::*;
use extern_timer::*;
use game::command::CommandSender;

#[derive(Clone)]
pub struct Guard {
    guard: extern_timer::Guard,
}

impl Guard {
    pub fn new(guard: extern_timer::Guard) -> Self {
        Self { guard }
    }
}

pub struct Timer {
    timer: MessageTimer<Command>,
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
            tags: HashMap::new(),
        }
    }

    pub fn repeating<S: ToString>(
        &mut self,
        duration: chrono::Duration,
        command: Command,
        tag: S,
    ) -> Guard {
        let guard = self.timer.schedule_repeating(duration, command);
        self.tags.insert(tag.to_string(), duration);
        Guard::new(guard)
    }
}

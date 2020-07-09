use super::*;
use extern_timer::*;
use game::{command::CommandSender, Rng};

#[derive(Clone)]
pub struct Guard {
    guard: Option<extern_timer::Guard>,
}

impl Guard {
    pub fn new(guard: Option<extern_timer::Guard>) -> Self {
        Self { guard }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping");
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alarm {
    duration_in_ticks: u128,
    command: Command,
    tag: String,
}

impl Alarm {
    pub fn new<S: ToString>(duration_in_ticks: u128, command: Command, tag: S) -> Self {
        Self {
            duration_in_ticks,
            command,
            tag: tag.to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TagType {
    Ticks(u128),
    Duration(chrono::Duration),
}

pub struct Timer {
    timer: MessageTimer<Command>,
    alarms: HashMap<u128, Vec<Alarm>>,
    current_tick: u128,
    command_tx: Option<CommandSender>,
    test_mode: bool,
    pub tags: HashMap<String, TagType>,
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
            command_tx: Some(command_tx),
            current_tick: 0,
            alarms: HashMap::new(),
            test_mode: false,
            tags: HashMap::new(),
        }
    }
    pub fn current_tick(&self) -> u128 {
        self.current_tick
    }

    pub fn repeating<S: ToString>(
        &mut self,
        duration: chrono::Duration,
        command: Command,
        tag: S,
    ) -> Guard {
        self.tags
            .insert(tag.to_string(), TagType::Duration(duration));

        if self.test_mode {
            Guard::new(None)
        } else {
            let guard = self.timer.schedule_repeating(duration, command);
            Guard::new(Some(guard))
        }
    }

    pub fn repeating_by_tick<S: ToString>(
        &mut self,
        duration_in_ticks: u128,
        command: Command,
        tag: S,
    ) {
        self.tags
            .insert(tag.to_string(), TagType::Ticks(duration_in_ticks));

        let alarm = Alarm::new(duration_in_ticks, command, tag);

        self.schedule_next(alarm, None);
    }

    pub fn stagger_repeating_by_tick<S: ToString>(
        &mut self,
        duration_in_ticks: u128,
        command: Command,
        tag: S,
        rng: &mut Rng,
    ) {
        self.tags
            .insert(tag.to_string(), TagType::Ticks(duration_in_ticks));

        let alarm = Alarm::new(duration_in_ticks, command, tag);
        self.schedule_next(alarm, Some(rng));
    }

    pub fn tick(&mut self, tick: u128) {
        self.current_tick = tick;

        match self.alarms.remove(&tick) {
            Some(alarms) => {
                for alarm in alarms {
                    self.trigger_alarm(&alarm);
                }
            }
            None => {}
        };
    }

    pub fn trigger<S: ToString>(&mut self, tag: S) {
        let tag = tag.to_string();

        let alarms = {
            self.alarms
                .iter()
                .flat_map(|(_, s)| s.iter().map(|a| a.clone()))
                .filter(|a| a.tag == tag)
                .collect::<Vec<Alarm>>()
        };

        for alarm in alarms {
            self.trigger_alarm(&alarm)
        }

        self.cancel(tag);
    }

    pub fn cancel<S: ToString>(&mut self, tag: S) {
        let tag = tag.to_string();
        for (_, alarm_set) in &mut self.alarms {
            alarm_set.retain(|a| a.tag != tag)
        }
    }

    fn trigger_alarm(&mut self, alarm: &Alarm) {
        self.schedule_next(alarm.clone(), None);
        Command::send(self.command_tx.clone(), alarm.command.clone())
    }

    fn schedule_next(&mut self, alarm: Alarm, rng: Option<&mut Rng>) {
        let target_tick = if let Some(rng) = rng {
            self.current_tick
                + rng.range(0, alarm.duration_in_ticks as i128, "timer_stagger") as u128
        } else {
            self.current_tick + alarm.duration_in_ticks
        };

        if self.alarms.contains_key(&target_tick) {
            self.alarms.get_mut(&target_tick).unwrap().push(alarm);
        } else {
            self.alarms.insert(target_tick, vec![alarm]);
        }
    }

    pub fn set_test_mode(&mut self) {
        self.test_mode = true;
    }
}

use super::*;
use extern_timer::*;
use game::Rng;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub struct Guard {
    guard: Option<extern_timer::Guard>,
    execute_flag: Option<Arc<AtomicBool>>,
    ignore_drop: bool,
}

impl Guard {
    pub fn new(guard: Option<extern_timer::Guard>, execute_flag: Option<Arc<AtomicBool>>) -> Self {
        Self {
            guard,
            execute_flag,
            ignore_drop: false,
        }
    }

    pub fn ignore(&mut self) {
        self.ignore_drop = true;
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(execute_flag) = self.execute_flag.clone() {
            if self.ignore_drop {
                return;
            }
            execute_flag.store(false, Ordering::Relaxed);
            println!("dropping");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alarm {
    duration_in_ticks: u128,
    command: Command,
    execute_flag: Arc<AtomicBool>,
    tag: String,
}

impl Alarm {
    pub fn new<S: ToString>(duration_in_ticks: u128, command: Command, tag: S) -> Self {
        Self {
            duration_in_ticks,
            command,
            execute_flag: Arc::new(AtomicBool::new(true)),
            tag: tag.to_string(),
        }
    }

    pub fn execute_flag(&self) -> Arc<AtomicBool> {
        self.execute_flag.clone()
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

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer")
            .field("alarms", &self.alarms)
            .field("current_tick", &self.current_tick)
            .field("test_mode", &self.test_mode)
            .field("command_tx", &self.command_tx)
            .finish()
    }
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

    pub fn repeating_by_time<S: ToString>(
        &mut self,
        duration: chrono::Duration,
        command: Command,
        tag: S,
    ) -> Guard {
        self.tags
            .insert(tag.to_string(), TagType::Duration(duration));

        if self.test_mode {
            Guard::new(None, None)
        } else {
            let guard = self.timer.schedule_repeating(duration, command);
            Guard::new(Some(guard), None)
        }
    }

    pub fn repeating_by_tick<S: ToString>(
        &mut self,
        duration_in_ticks: u128,
        command: Command,
        tag: S,
    ) -> Guard {
        self.tags
            .insert(tag.to_string(), TagType::Ticks(duration_in_ticks));

        let alarm = Alarm::new(duration_in_ticks, command, tag);

        let execute_flag = alarm.clone().execute_flag();

        self.schedule_next(alarm, None, None);

        Guard::new(None, Some(execute_flag))
    }

    pub fn repeating_by_tick_starting_at<S: ToString>(
        &mut self,
        first: u128,
        duration_in_ticks: u128,
        command: Command,
        tag: S,
    ) -> Guard {
        self.tags
            .insert(tag.to_string(), TagType::Ticks(duration_in_ticks));

        let alarm = Alarm::new(duration_in_ticks, command, tag);

        let execute_flag = alarm.clone().execute_flag();

        self.schedule_next(alarm, Some(first), None);

        Guard::new(None, Some(execute_flag))
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
        self.schedule_next(alarm, None, Some(rng));
    }

    pub fn tick(&mut self, tick: u128) {
        let old_tick = self.current_tick;

        self.current_tick = tick;

        if self.current_tick - old_tick != 1 {
            let alarms = self.alarms.clone();
            let expired_alarms: Vec<&u128> =
                { alarms.keys().filter(|a| **a < tick).collect::<Vec<&u128>>() };

            for alarm in expired_alarms {
                let alarms = &self.alarms[alarm];

                #[allow(mutable_borrow_reservation_conflict)]
                self.trigger_alarms(alarms.clone());
            }
        }

        match self.alarms.remove(&tick) {
            Some(alarms) => {
                self.trigger_alarms(alarms);
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

        self.cancel(tag);

        for alarm in alarms {
            self.trigger_alarm(&alarm);
        }
    }

    pub fn cancel<S: ToString>(&mut self, tag: S) {
        let tag = tag.to_string();
        for (_, alarm_set) in &mut self.alarms {
            alarm_set.retain(|a| a.tag != tag);
        }
        self.tags.remove(&tag);
    }

    fn trigger_alarm(&mut self, alarm: &Alarm) {
        if !alarm.execute_flag.load(Ordering::Relaxed) {
            return;
        }

        self.schedule_next(alarm.clone(), None, None);
        Command::send(self.command_tx.clone(), alarm.command.clone())
    }

    fn trigger_alarms(&mut self, alarms: Vec<Alarm>) {
        for alarm in alarms {
            self.trigger_alarm(&alarm);
        }
    }

    fn schedule_next(&mut self, alarm: Alarm, first: Option<u128>, rng: Option<&mut Rng>) {
        let target_tick = if let Some(tick) = first {
            tick
        } else if let Some(rng) = rng {
            rng.range(0, alarm.duration_in_ticks as i128, "timer_stagger") as u128
        } else {
            alarm.duration_in_ticks
        } + self.current_tick;

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

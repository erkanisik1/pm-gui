#[derive(Debug, Clone)]
pub enum AppEvent {
    CategorySelected(String),
    ComponentSelected(String),
    ShowSettings,
}

pub struct EventManager {
    events: Vec<AppEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: AppEvent) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<AppEvent> {
        self.events.pop()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

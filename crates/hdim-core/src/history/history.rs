use crate::Adjustments;

#[derive(Debug, Clone)]
pub struct History {
    states: Vec<Adjustments>,
    current_index: usize,
}

impl History {
    pub fn new(initial_adjustments: Adjustments) -> Self {
        History {
            states: vec![initial_adjustments],
            current_index: 0,
        }
    }

    pub fn record_adjustments(&mut self, new_adjustments: Adjustments) {
        // If we are not at the end of the history, truncate the future states
        if self.current_index < self.states.len() - 1 {
            self.states.truncate(self.current_index + 1);
        }
        self.states.push(new_adjustments);
        self.current_index = self.states.len() - 1;
    }

    pub fn undo(&mut self) -> Option<Adjustments> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(self.states[self.current_index])
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<Adjustments> {
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            Some(self.states[self.current_index])
        } else {
            None
        }
    }

    pub fn current_adjustments(&self) -> Adjustments {
        self.states[self.current_index]
    }
}

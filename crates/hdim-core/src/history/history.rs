//! Undo/Redo history management.
//!
//! Tracks a stack of [Adjustments] states, allowing the user to navigate
//! backward and forward through their editing session.

use crate::Adjustments;

/// Manages the history of image adjustments.
///
/// Implements a linear undo/redo stack. When a new state is recorded,
/// any "future" states (created by undoing) are discarded.
#[derive(Debug, Clone)]
pub struct History {
    /// The stack of recorded adjustment states.
    states: Vec<Adjustments>,
    /// The index of the currently active state in `states`.
    current_index: usize,
}

impl History {
    /// Creates a new history stack with an initial state.
    ///
    /// # Arguments
    ///
    /// * `initial_adjustments` - The starting [Adjustments] configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdim_core::{HdimImage, Adjustments};
    /// use hdim_core::history::history::History;
    ///
    /// let initial = Adjustments::default();
    /// let history = History::new(initial);
    /// ```
    pub fn new(initial_adjustments: Adjustments) -> Self {
        History {
            states: vec![initial_adjustments],
            current_index: 0,
        }
    }

    /// Records a new adjustment state, pushing it onto the history stack.
    ///
    /// If the current state is not the latest (i.e., after an undo),
    /// all future states are discarded before adding the new one.
    ///
    /// # Arguments
    ///
    /// * `new_adjustments` - The new [Adjustments] state to record.
    pub fn record_adjustments(&mut self, new_adjustments: Adjustments) {
        // If we are not at the end of the history, truncate the future states
        if self.current_index < self.states.len() - 1 {
            self.states.truncate(self.current_index + 1);
        }
        self.states.push(new_adjustments);
        self.current_index = self.states.len() - 1;
    }

    /// Moves one step backward in history.
    ///
    /// # Returns
    ///
    /// * `Some(Adjustments)` - The previous state, if available.
    /// * `None` - If already at the initial state.
    pub fn undo(&mut self) -> Option<Adjustments> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(self.states[self.current_index])
        } else {
            None
        }
    }

    /// Moves one step forward in history.
    ///
    /// # Returns
    ///
    /// * `Some(Adjustments)` - The next state, if available (i.e., after an undo).
    /// * `None` - If already at the latest state.
    pub fn redo(&mut self) -> Option<Adjustments> {
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            Some(self.states[self.current_index])
        } else {
            None
        }
    }

    /// Returns the currently active adjustment state.
    pub fn current_adjustments(&self) -> Adjustments {
        self.states[self.current_index]
    }

    /// Returns the current index in the history stack.
    pub fn current_index(&self) -> usize {
        self.current_index
    }
}

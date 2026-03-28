//! Undo/Redo history management.
//!
//! Tracks a stack of image states, allowing the user to navigate
//! backward and forward through their editing session.

use crate::Adjustments;
use image::DynamicImage;
use std::sync::Arc;

/// Represents a single state in the history stack.
#[derive(Debug, Clone)]
pub struct HistoryState {
    /// The image data at this point in history.
    pub data: Arc<DynamicImage>,
    /// The adjustments at this point in history.
    pub adjustments: Adjustments,
}

/// Manages the history of image states.
///
/// Implements a linear undo/redo stack. When a new state is recorded,
/// any "future" states (created by undoing) are discarded.
#[derive(Debug, Clone)]
pub struct History {
    /// The stack of recorded states.
    states: Vec<HistoryState>,
    /// The index of the currently active state in `states`.
    current_index: usize,
}

impl History {
    /// Creates a new history stack with an initial state.
    ///
    /// # Arguments
    ///
    /// * `initial_image` - The starting image data.
    /// * `initial_adjustments` - The starting [Adjustments] configuration.
    pub fn new(initial_image: Arc<DynamicImage>, initial_adjustments: Adjustments) -> Self {
        History {
            states: vec![HistoryState {
                data: initial_image,
                adjustments: initial_adjustments,
            }],
            current_index: 0,
        }
    }

    /// Records a new state, pushing it onto the history stack.
    ///
    /// If the current state is not the latest (i.e., after an undo),
    /// all future states are discarded before adding the new one.
    ///
    /// # Arguments
    ///
    /// * `image` - The new image data to record.
    /// * `adjustments` - The new [Adjustments] state to record.
    pub fn record_state(&mut self, image: Arc<DynamicImage>, adjustments: Adjustments) {
        // If we are not at the end of the history, truncate the future states
        if self.current_index < self.states.len() - 1 {
            self.states.truncate(self.current_index + 1);
        }
        self.states.push(HistoryState {
            data: image,
            adjustments,
        });
        self.current_index = self.states.len() - 1;
    }

    /// Moves one step backward in history.
    ///
    /// # Returns
    ///
    /// * `Some(HistoryState)` - The previous state, if available.
    /// * `None` - If already at the initial state.
    pub fn undo(&mut self) -> Option<HistoryState> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(self.states[self.current_index].clone())
        } else {
            None
        }
    }

    /// Moves one step forward in history.
    ///
    /// # Returns
    ///
    /// * `Some(HistoryState)` - The next state, if available (i.e., after an undo).
    /// * `None` - If already at the latest state.
    pub fn redo(&mut self) -> Option<HistoryState> {
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            Some(self.states[self.current_index].clone())
        } else {
            None
        }
    }

    /// Returns the currently active state.
    pub fn current_state(&self) -> HistoryState {
        self.states[self.current_index].clone()
    }

    /// Returns the current index in the history stack.
    pub fn current_index(&self) -> usize {
        self.current_index
    }
}

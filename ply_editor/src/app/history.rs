//! History - Undo/redo management

use crate::app::commands::Command;
use std::collections::VecDeque;

/// Maximum number of commands to keep in history
const MAX_HISTORY: usize = 100;

/// History manager for undo/redo
pub struct History {
    undo_stack: VecDeque<Box<dyn Command>>,
    redo_stack: VecDeque<Box<dyn Command>>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    /// Execute a command and add it to the undo stack
    pub fn execute(&mut self, mut command: Box<dyn Command>, scene: &mut crate::data::Scene) {
        command.apply(scene);
        
        // Add to undo stack
        self.undo_stack.push_back(command);
        
        // Clear redo stack when new command is executed
        self.redo_stack.clear();
        
        // Limit history size
        while self.undo_stack.len() > MAX_HISTORY {
            self.undo_stack.pop_front();
        }
    }

    /// Undo the last command
    pub fn undo(&mut self, scene: &mut crate::data::Scene) -> bool {
        if let Some(mut command) = self.undo_stack.pop_back() {
            command.undo(scene);
            self.redo_stack.push_back(command);
            true
        } else {
            false
        }
    }

    /// Redo the last undone command
    pub fn redo(&mut self, scene: &mut crate::data::Scene) -> bool {
        if let Some(mut command) = self.redo_stack.pop_back() {
            command.apply(scene);
            self.undo_stack.push_back(command);
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of commands in undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
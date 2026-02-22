use crate::configuration::Configuration;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Context<'a> {
  pub config: &'a Configuration,
  pub text: &'a str,
  handled_comments: HashSet<usize>,
  /// Maps comment source position → number of spaces to insert before it.
  /// Used by Smart alignment mode.
  comment_spaces: HashMap<usize, usize>,
}

impl<'a> Context<'a> {
  pub fn new(text: &'a str, config: &'a Configuration) -> Self {
    Self {
      config,
      text,
      handled_comments: HashSet::new(),
      comment_spaces: HashMap::new(),
    }
  }

  pub fn has_handled_comment(&self, pos: usize) -> bool {
    self.handled_comments.contains(&pos)
  }

  pub fn add_handled_comment(&mut self, pos: usize) {
    self.handled_comments.insert(pos);
  }

  /// Get the pre-computed number of spaces before a trailing comment (smart mode).
  pub fn get_comment_spaces(&self, pos: usize) -> Option<usize> {
    self.comment_spaces.get(&pos).copied()
  }

  /// Set the pre-computed number of spaces before a trailing comment.
  pub fn set_comment_spaces(&mut self, pos: usize, spaces: usize) {
    self.comment_spaces.insert(pos, spaces);
  }

  pub fn get_line_number_at_pos(&self, pos: usize) -> usize {
    // todo: make this faster by using an array of line indexes
    let mut line_number = 0;
    for (i, c) in self.text.char_indices() {
      if pos <= i {
        break;
      }
      if c == '\n' {
        line_number += 1;
      }
    }
    line_number
  }
}

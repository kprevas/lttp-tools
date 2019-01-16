use std::collections::HashMap;

use super::command::ParameterizedCommand;
use super::track::Track;

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub track_idx: usize,
    pub cmd_idx: usize,
    pub repeat_count: u8,
}

pub struct SeqTree {
    children: HashMap<ParameterizedCommand, SeqTree>,
    locations: Vec<Location>,
}

#[derive(Debug)]
pub struct Sequence {
    pub commands: Vec<ParameterizedCommand>,
    pub locations: Vec<Location>,
}

impl SeqTree {
    pub fn new() -> SeqTree {
        SeqTree {
            children: HashMap::new(),
            locations: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: &Track, track_idx: usize) {
        for i in 0..track.commands.len() {
            self.add_commands(&track.commands[i..track.commands.len()], track_idx, i);
        }
    }

    fn add_commands(
        &mut self,
        commands: &[ParameterizedCommand],
        track_idx: usize,
        seq_start: usize,
    ) {
        self.locations.push(Location {
            track_idx,
            cmd_idx: seq_start,
            repeat_count: 1,
        });
        if !commands.is_empty() {
            let command = &commands[0];
            if command.call_loop_eligible() {
                let child;
                if self.children.contains_key(command) {
                    child = self.children.get_mut(command).unwrap();
                } else {
                    self.children.insert(command.clone(), SeqTree::new());
                    child = self.children.get_mut(command).unwrap();
                }
                child.add_commands(&commands[1..commands.len()], track_idx, seq_start);
            }
        }
    }

    pub fn valid_sequences<'a>(&'a self) -> impl Iterator<Item = Sequence> + 'a {
        struct StackItem<'a> {
            node: &'a SeqTree,
            command: Option<&'a ParameterizedCommand>,
            child_idx: usize,
        }

        struct TreeIter<'a> {
            stack: Vec<StackItem<'a>>,
        }

        impl<'a> Iterator for TreeIter<'a> {
            type Item = Sequence;

            fn next(&mut self) -> Option<Sequence> {
                if self.stack.is_empty() {
                    None
                } else {
                    let mut top = self.stack.pop().unwrap();
                    while top.child_idx == top.node.children.len() {
                        match self.stack.pop() {
                            None => return None,
                            Some(item) => top = item,
                        }
                    }
                    let (cmd, child) = top.node.children.iter().nth(top.child_idx).unwrap();
                    top.child_idx += 1;
                    self.stack.push(top);
                    self.stack.push(StackItem {
                        node: child,
                        command: Some(cmd),
                        child_idx: 0,
                    });
                    let commands: Vec<ParameterizedCommand> = self
                        .stack
                        .iter()
                        .filter_map(|stack_item| stack_item.command)
                        .map(Clone::clone)
                        .collect();
                    let commands_len = commands.len();
                    Some(Sequence {
                        commands,
                        locations: SeqTree::consolidate_locations(&child.locations, commands_len),
                    })
                }
            }
        }

        let mut iter = TreeIter { stack: Vec::new() };
        iter.stack.push(StackItem {
            node: self,
            command: None,
            child_idx: 0,
        });
        iter.filter(|seq| {
            seq.commands.len() > 3
                && seq
                    .locations
                    .iter()
                    .fold(0, |acc, &loc| acc + loc.repeat_count)
                    > 1
        })
    }

    fn consolidate_locations(locations: &Vec<Location>, seq_length: usize) -> Vec<Location> {
        let mut consolidated = Vec::new();
        for location in locations {
            if consolidated.last().map_or(false, |loc: &Location| {
                loc.track_idx == location.track_idx
                    && loc.cmd_idx + seq_length * loc.repeat_count as usize > location.cmd_idx
            }) {
                // too close, skip
            } else if consolidated.last().map_or(false, |loc: &Location| {
                loc.track_idx == location.track_idx
                    && loc.cmd_idx + seq_length * loc.repeat_count as usize == location.cmd_idx
            }) {
                consolidated.last_mut().unwrap().repeat_count += 1;
            } else {
                consolidated.push(location.clone());
            }
        }
        consolidated
    }
}

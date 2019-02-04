use super::command::ParameterizedCommand;
use super::track::Track;

#[cfg(test)]
mod tests {
    use super::super::command::Command;
    use super::*;

    fn convert_commands(commands: &[Command]) -> Vec<ParameterizedCommand> {
        commands
            .iter()
            .map(|cmd| ParameterizedCommand::new(None, None, cmd.clone()))
            .collect::<Vec<ParameterizedCommand>>()
    }

    fn assert_outgoing_edge(tree: &SeqTree, commands: &[Command]) {
        assert!(get_outgoing_edge(tree, commands).is_some());
    }

    fn get_outgoing_edge<'a>(tree: &'a SeqTree, commands: &[Command]) -> Option<&'a Edge> {
        tree.edges
            .iter()
            .find(|edge| edge.commands == convert_commands(commands))
    }

    fn assert_seq(seq: &Sequence, commands: &[Command]) {
        assert_eq!(seq.commands, convert_commands(commands));
    }

    #[test]
    fn test_no_overlaps() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                ],
            },
            0,
        );
        assert_eq!(3, tree.edges.len());
        assert_outgoing_edge(
            &tree,
            &[Command::Note(1), Command::Note(2), Command::Note(3)],
        );
        assert_outgoing_edge(&tree, &[Command::Note(2), Command::Note(3)]);
        assert_outgoing_edge(&tree, &[Command::Note(3)]);
    }

    #[test]
    fn test_overlap() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                ],
            },
            0,
        );
        assert_eq!(2, tree.edges.len());
        assert_outgoing_edge(&tree, &[Command::Note(1)]);
        assert_outgoing_edge(&tree, &[Command::Note(2)]);
        let child = get_outgoing_edge(&tree, &[Command::Note(1)])
            .unwrap()
            .target
            .as_ref()
            .unwrap();
        assert_eq!(2, child.edges.len());
        assert_outgoing_edge(child, &[Command::Note(1), Command::Note(2)]);
        assert_outgoing_edge(child, &[Command::Note(2)]);
    }

    #[test]
    fn test_repeat() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                ],
            },
            0,
        );
        let child = get_outgoing_edge(&tree, &[Command::Note(1), Command::Note(2)])
            .unwrap()
            .target
            .as_ref()
            .unwrap();
        assert_eq!(1, child.locations.len());
        assert_eq!(0, child.locations[0].cmd_idx);
        assert_eq!(2, child.locations[0].repeat_count);
    }

    #[test]
    fn test_ineligible_command() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::CallLoop(0, 0)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                ],
            },
            0,
        );
        assert_eq!(2, tree.edges.len());
        assert_outgoing_edge(&tree, &[Command::Note(1)]);
        assert_outgoing_edge(&tree, &[Command::Note(2)]);
    }

    #[test]
    fn test_best_seq_none() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                ],
            },
            0,
        );
        assert!(tree.best_sequence().is_none());
    }

    #[test]
    fn test_best_seq_no_repeats() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                    ParameterizedCommand::new(None, None, Command::Note(5)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                ],
            },
            0,
        );
        let seq = tree.best_sequence().unwrap();
        assert_seq(
            &seq,
            &[
                Command::Note(1),
                Command::Note(2),
                Command::Note(3),
                Command::Note(4),
            ],
        );
        assert_eq!(
            seq.locations,
            vec![
                Location {
                    track_idx: 0,
                    cmd_idx: 0,
                    repeat_count: 1,
                },
                Location {
                    track_idx: 0,
                    cmd_idx: 5,
                    repeat_count: 1,
                }
            ]
        );
    }

    #[test]
    fn test_best_seq_repeats() {
        let mut tree = SeqTree::new();
        tree.add_track(
            &Track {
                commands: vec![
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                    ParameterizedCommand::new(None, None, Command::Note(5)),
                    ParameterizedCommand::new(None, None, Command::Note(6)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                    ParameterizedCommand::new(None, None, Command::Note(5)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                    ParameterizedCommand::new(None, None, Command::Note(1)),
                    ParameterizedCommand::new(None, None, Command::Note(2)),
                    ParameterizedCommand::new(None, None, Command::Note(3)),
                    ParameterizedCommand::new(None, None, Command::Note(4)),
                ],
            },
            0,
        );
        let seq = tree.best_sequence().unwrap();
        assert_seq(
            &seq,
            &[
                Command::Note(1),
                Command::Note(2),
                Command::Note(3),
                Command::Note(4),
            ],
        );
        assert_eq!(
            seq.locations,
            vec![
                Location {
                    track_idx: 0,
                    cmd_idx: 0,
                    repeat_count: 1,
                },
                Location {
                    track_idx: 0,
                    cmd_idx: 6,
                    repeat_count: 1,
                },
                Location {
                    track_idx: 0,
                    cmd_idx: 11,
                    repeat_count: 3,
                }
            ]
        );
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location {
    pub track_idx: usize,
    pub cmd_idx: usize,
    pub repeat_count: u8,
}

#[derive(Debug)]
struct Edge {
    commands: Vec<ParameterizedCommand>,
    target: Option<SeqTree>,
}

#[derive(Debug)]
pub struct SeqTree {
    edges: Vec<Edge>,
    locations: Vec<Location>,
}

#[derive(Debug)]
pub struct Sequence {
    pub commands: Vec<ParameterizedCommand>,
    pub locations: Vec<Location>,
}

fn add_location(locations: &mut Vec<Location>, track_idx: usize, cmd_idx: usize, seq_len: usize) {
    let mut found = false;
    {
        let adjoining_location = locations
            .iter_mut()
            .find(|loc| loc.track_idx == track_idx && loc.cmd_idx + seq_len == cmd_idx);
        if let Some(loc) = adjoining_location {
            loc.repeat_count += 1;
            found = true;
        }
    }
    if !found {
        locations.push(Location {
            track_idx,
            cmd_idx,
            repeat_count: 1,
        });
    }
}

impl SeqTree {
    pub fn new() -> SeqTree {
        SeqTree {
            edges: Vec::new(),
            locations: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: &Track, track_idx: usize) {
        for i in 0..track.commands.len() {
            self.add_commands(&track.commands[i..track.commands.len()], track_idx, i, 0);
        }
    }

    fn add_commands(
        &mut self,
        commands: &[ParameterizedCommand],
        track_idx: usize,
        seq_start: usize,
        seq_len: usize,
    ) {
        if seq_len > 0 {
            add_location(&mut self.locations, track_idx, seq_start, seq_len);
        }
        if !commands.is_empty() {
            let mut found_edge = false;
            {
                let edge = self
                    .edges
                    .iter_mut()
                    .find(|edge| edge.commands[0] == commands[0]);
                if let Some(edge) = edge {
                    found_edge = true;
                    let shared_prefix = commands
                        .iter()
                        .enumerate()
                        .take_while(|&(idx, cmd)| {
                            edge.commands.len() > idx && *cmd == edge.commands[idx]
                        })
                        .map(|(_, e)| e)
                        .collect::<Vec<&ParameterizedCommand>>();
                    let shared_seq_len = seq_len + shared_prefix.len();
                    if shared_prefix.len() == edge.commands.len() {
                        add_location(
                            &mut edge.target.as_mut().unwrap().locations,
                            track_idx,
                            seq_start,
                            shared_seq_len,
                        );
                        if shared_prefix.len() < commands.len() {
                            edge.target.as_mut().unwrap().add_commands(
                                &commands[shared_prefix.len()..],
                                track_idx,
                                seq_start,
                                shared_seq_len,
                            );
                        }
                    } else {
                        let mut new_target = SeqTree {
                            edges: Vec::new(),
                            locations: Vec::new(),
                        };
                        let old_target = edge.target.take();
                        if let Some(old_target) = old_target {
                            for loc in &old_target.locations {
                                add_location(
                                    &mut new_target.locations,
                                    loc.track_idx,
                                    loc.cmd_idx,
                                    shared_seq_len,
                                );
                            }
                            let mut old_suffix = Vec::new();
                            old_suffix.extend_from_slice(&edge.commands[shared_prefix.len()..]);
                            new_target.edges.push(Edge {
                                commands: old_suffix,
                                target: Some(old_target),
                            });
                        }
                        edge.commands = shared_prefix.iter().map(|&cmd| cmd.clone()).collect();
                        edge.target.replace(new_target);
                        edge.target.as_mut().unwrap().add_commands(
                            &commands[shared_prefix.len()..],
                            track_idx,
                            seq_start,
                            shared_seq_len,
                        );
                    }
                }
            }
            if !found_edge {
                let mut rest_commands = Vec::<ParameterizedCommand>::new();
                rest_commands.extend(
                    commands
                        .iter()
                        .take_while(|cmd| cmd.call_loop_eligible())
                        .map(|cmd| cmd.clone()),
                );
                if !rest_commands.is_empty() {
                    let rest_len = seq_len + rest_commands.len();
                    let target = SeqTree {
                        edges: Vec::new(),
                        locations: Vec::new(),
                    };
                    self.edges.push(Edge {
                        commands: rest_commands,
                        target: Some(target),
                    });
                    let target = self.edges.last_mut().unwrap().target.as_mut().unwrap();
                    target.add_commands(&[], track_idx, seq_start, rest_len);
                }
            }
        }
    }

    pub fn best_sequence(&self) -> Option<Sequence> {
        fn score(seq: &Sequence) -> usize {
            let commands_replaced = seq
                .locations
                .iter()
                .fold(0, |acc, &loc| acc + loc.repeat_count as usize)
                * seq.commands.len();
            let commands_used = seq.commands.len() + seq.locations.len();
            commands_replaced - commands_used
        }

        struct StackItem<'a> {
            node: &'a SeqTree,
            commands: Vec<ParameterizedCommand>,
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
                    while top.child_idx == top.node.edges.len() {
                        match self.stack.pop() {
                            None => return None,
                            Some(item) => top = item,
                        }
                    }
                    let edge = &top.node.edges[top.child_idx];
                    top.child_idx += 1;
                    let mut commands = top.commands.clone();
                    self.stack.push(top);
                    commands.extend_from_slice(edge.commands.as_ref());
                    let target = edge.target.as_ref().unwrap();
                    let commands_len = commands.len();
                    self.stack.push(StackItem {
                        node: target,
                        commands: commands.clone(),
                        child_idx: 0,
                    });
                    Some(Sequence {
                        commands,
                        locations: SeqTree::consolidate_locations(&target.locations, commands_len),
                    })
                }
            }
        }

        let mut iter = TreeIter { stack: Vec::new() };
        iter.stack.push(StackItem {
            node: self,
            commands: vec![],
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
        .max_by_key(score)
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

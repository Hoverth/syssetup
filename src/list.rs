
use crate::theme::*;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ego_tree::{tree, NodeId};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState},
    Frame,
};

struct ListNode {
    name: &'static str,
    command: &'static str,
}

/// This is a data structure that has everything necessary to draw and manage a menu of commands
pub struct CustomList {
    /// The tree data structure, to represent regular items
    /// and "directories"
    inner_tree: ego_tree::Tree<ListNode>,
    /// This stack keeps track of our "current dirrectory". You can think of it as `pwd`. but not
    /// just the current directory, all paths that took us here, so we can "cd .."
    visit_stack: Vec<NodeId>,
    /// This is the state asociated with the list widget, used to display the selection in the
    /// widget
    list_state: ListState,
}

impl CustomList {
    /// It's really easy to make this accept a tree, and make it reusable, but rn its only called
    /// once, so I didn't bother, and it gets initialized here
    pub fn new() -> Self {
        // When a function call ends with an exclamation mark, it means it's a macro, like in this
        // case the tree! macro expands to `ego-tree::tree` data structure
        let tree = tree!(ListNode {
            name: "root",
            command: ""
        } => {
            ListNode {
                name: "Please select a command: ([R] designates scripts that require running with root)",
                command: "echo \"this command does nothing...\""
            },
            ListNode {
                name: "First install scripts",
                command: ""
            } => {
                ListNode {
                    name: "[R] Install the extra firmware packages",
                    command: include_str!("commands/install-extra-firmware.sh")
                },
                ListNode {
                    name: "[R] Setup cli utilities (stow, tmux, fzf, zsh, etc)",
                    command: include_str!("commands/setup-cli.sh"),
                },
                ListNode {
                    name: "[R] Setup MPD + ncmpcpp",
                    command: include_str!("commands/setup-mpd.sh"),
                },
                ListNode {
                    name: "[R] Setup grabbers (yt-dlp, gallery-dl)",
                    command: include_str!("commands/setup-grabbers.sh"),
                },
                ListNode {
                    name: "[R] Setup Chaotic-AUR [to test]",
                    command: include_str!("commands/setup-caur.sh"),
                },
                ListNode {
                    name: "[R] Setup TUIs (neomutt, iamb)",
                    command: include_str!("commands/setup-tuis.sh"), 
                },
                ListNode {
                    name: "[R] Install all GUI programs",
                    command: include_str!("commands/setup-gui.sh")
                },
                ListNode {
                    name: "[R] Install Flatpak",
                    command: include_str!("commands/install-flatpak.sh")
                },
                ListNode {
                    name: "Install Flatpak Applications",
                    command: include_str!("commands/install-flatpak-apps.sh")
                },
                ListNode {
                    name: "Create example fstab",
                    command: "echo creating example-fstab.txt... \necho \"UUID=<> /home/<> btrfs subvol=<>,defaults,noatime,noautodefrag,ssd 0 0\" > example-fstab.txt"
                }
            },
            ListNode {
                name: "Rust",
                command: ""
            } => {
                ListNode {
                    name: "Install Rust",
                    command: "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && rustup update"
                },
                ListNode {
                    name: "Install yazi",
                    command: "cargo install --locked yazi-fm yazi-cli"
                },
                ListNode {
                    name: "Install macchina",
                    command: "cargo install macchina"
                },
                ListNode {
                    name: "Install ctow",
                    command: "cargo install ctow"
                },
                ListNode {
                    name: "Install iamb",
                    command: "cargo install --locked iamb"
                },
                ListNode {
                    name: "Install netscanner",
                    command: "cargo install netscanner"
                }
            },
            ListNode {
                name: "Tweaks & Misc",
                command: ""
            } => {
                ListNode {
                    name: "Setup Librewolf native messaging symlink",
                    command: include_str!("commands/setup-librewolf.sh")
                },
                ListNode {
                    name: "Show command to let user write to serial ports",
                    command: "echo \"'sudo usermod -aG $USER:$USER GROUP' where group is either uucp for arch or dialout for debian\""
                }
            }
        });
        // We don't get a reference, but rather an id, because references are siginficantly more
        // paintfull to manage
        let root_id = tree.root().id();
        Self {
            inner_tree: tree,
            visit_stack: vec![root_id],
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    /// Draw our custom widget to the frame
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        // Get the last element in the `visit_stack` vec
        let theme = get_theme();
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();
        let mut items = vec![];

        let title = if !self.at_root() {
            format!(" SysSetup - {} ", curr.value().name)
        } else {
            " SysSetup ".to_string()
        };

        // If we are not at the root of our filesystem tree, we need to add `..` path, to be able
        // to go up the tree
        // icons:   
        if !self.at_root() {
            items.push(Line::from(format!("{}  ..", theme.dir_icon)).style(theme.dir_color));
        }

        // Iterate through all the children
        for node in curr.children() {
            // The difference between a "directory" and a "command" is simple: if it has children,
            // it's a directory and will be handled as such
            if node.has_children() {
                items.push(
                    Line::from(format!("{}  {}", theme.dir_icon, node.value().name))
                        .style(theme.dir_color),
                );
            } else {
                items.push(
                    Line::from(format!("{}  {}", theme.cmd_icon, node.value().name))
                        .style(theme.cmd_color),
                );
            }
        }

        // create the normal list widget containing only item in our "working directory" / tree
        // node
        let list = List::new(items)
            .highlight_style(Style::default().reversed())
            .block(Block::default().borders(Borders::ALL).title(title))
            .scroll_padding(1);

        // Render it
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Handle key events, we are only interested in `Press` and `Repeat` events
    pub fn handle_key(&mut self, event: KeyEvent) -> Option<&'static str> {
        if event.kind == KeyEventKind::Release {
            return None;
        }
        match event.code {
            // Damm you Up arrow, use vim lol
            KeyCode::Char('j') | KeyCode::Down => {
                self.try_scroll_down();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.try_scroll_up();
                None
            }
            KeyCode::Enter => self.handle_enter(),
            _ => None,
        }
    }
    fn try_scroll_up(&mut self) {
        self.list_state
            .select(Some(self.list_state.selected().unwrap().saturating_sub(1)));
    }
    fn try_scroll_down(&mut self) {
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();

        let count = curr.children().count();

        let curr_selection = self.list_state.selected().unwrap();
        if self.at_root() {
            self.list_state
                .select(Some((curr_selection + 1).min(count - 1)));
        } else {
            // When we are not at the root, we have to account for 1 more "virtual" node, `..`. So
            // the count is 1 bigger (select is 0 based, because it's an index)
            self.list_state
                .select(Some((curr_selection + 1).min(count)));
        }
    }

    /// Handles the <Enter> key. This key can do 3 things:
    /// - Run a command, if it is the currently selected item,
    /// - Go up a directory
    /// - Go down into a directory
    /// Returns `Some(command)` when command is selected, othervise we returns `None`
    fn handle_enter(&mut self) -> Option<&'static str> {
        // Get the current node (current directory)
        let curr = self
            .inner_tree
            .get(*self.visit_stack.last().unwrap())
            .unwrap();
        let selected = self.list_state.selected().unwrap();

        // if we are not at the root, and the first element is selected,
        // we can be sure it's '..', so we go up the directory
        if !self.at_root() && selected == 0 {
            self.visit_stack.pop();
            self.list_state.select(Some(0));
            return None;
        }

        for (mut idx, node) in curr.children().enumerate() {
            // at this point, we know that we are not on the .. item, and our indexes of the items never had ..
            // item. so to balance it out, in case the selection index contains .., se add 1 to our node index
            if !self.at_root() {
                idx += 1;
            }
            if idx == selected {
                if node.has_children() {
                    self.visit_stack.push(node.id());
                    self.list_state.select(Some(0));
                    return None;
                } else {
                    return Some(node.value().command);
                }
            }
        }
        None
    }

    /// Checks weather the current tree node is the root node (can we go up the tree or no)
    /// Returns `true` if we can't go up the tree (we are at the tree root)
    /// else returns `false`
    fn at_root(&self) -> bool {
        self.visit_stack.len() == 1
    }
}

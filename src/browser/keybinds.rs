use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum VimAction {
    ScrollDown,
    ScrollDownPage,
    ScrollUp,
    ScrollUpPage,
    ScrollLeft,
    ScrollRight,
    GoToTop,
    GoToBottom,
    SearchMode,
    YankUrl,
    EnterInsertMode,
    LeaveInsertMode,
    GoToPrevious,
    GoToNext,
    OpenDevTools,
    ShowOverlay,
    Refresh,
}

pub struct KeybindingManager {
    sequence: String,
    bindings: HashMap<String, VimAction>,
    in_insert_mode: bool,
    show_overlay: bool,
}

impl KeybindingManager {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();

        bindings.insert("j".into(), VimAction::ScrollDown);
        bindings.insert("k".into(), VimAction::ScrollUp);
        bindings.insert("h".into(), VimAction::ScrollLeft);
        bindings.insert("l".into(), VimAction::ScrollRight);
        bindings.insert("d".into(), VimAction::ScrollDownPage);
        bindings.insert("u".into(), VimAction::ScrollUpPage);
        bindings.insert("gg".into(), VimAction::GoToTop);
        bindings.insert("G".into(), VimAction::GoToBottom);
        bindings.insert("/".into(), VimAction::SearchMode);
        bindings.insert("i".into(), VimAction::EnterInsertMode);
        bindings.insert("yy".into(), VimAction::YankUrl);
        bindings.insert("<Esc>".into(), VimAction::LeaveInsertMode);
        bindings.insert("D".into(), VimAction::OpenDevTools);
        bindings.insert("H".into(), VimAction::GoToPrevious);
        bindings.insert("L".into(), VimAction::GoToNext);
        bindings.insert("f".into(), VimAction::ShowOverlay);
        bindings.insert("r".into(), VimAction::Refresh);
        Self {
            sequence: String::new(),
            bindings,
            in_insert_mode: false,
            show_overlay: false,
        }
    }

    pub fn push_key(&mut self, key: &str) -> Option<VimAction> {
        if self.in_insert_mode {
            if key == "<Esc>" || key == "\u{1b}" {
                self.in_insert_mode = false;
                self.sequence.clear();
                return Some(VimAction::LeaveInsertMode);
            }
            return None;
        }

        self.sequence.push_str(key);

        println!("Current sequence: {}", self.sequence);

        if let Some(action) = self.bindings.get(&self.sequence) {
            if let VimAction::EnterInsertMode = action {
                self.in_insert_mode = true;
            }
            self.sequence.clear();
            return Some(action.clone());
        }

        if !self.bindings.keys().any(|k| k.starts_with(&self.sequence)) {
            self.sequence.clear();
        }

        None
    }

    pub fn is_insert_mode(&self) -> bool {
        self.in_insert_mode
    }

    pub fn set_insert_mode(&mut self, mode: bool) {
        self.in_insert_mode = mode;
        if !mode {
            self.sequence.clear();
        }
    }

    pub fn is_show_overlay(&self) -> bool {
        self.show_overlay
    }

    pub fn set_show_overlay(&mut self, show: bool) {
        self.show_overlay = show;
        if !show {
            self.sequence.clear();
        }
    }
}

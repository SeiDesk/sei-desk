use super::{Entry, Focus, Model, preferences::Pin};
use std::sync::Arc;
impl Focus {
    pub fn id(self) -> &'static str {
        match self {
            Self::Search => "applications-search",
            Self::Filter => "apps-filter",
            Self::Presentation => "apps-view",
            Self::Sort => "apps-sort",
            Self::Group => "apps-group",
            Self::Results => "applications-results-panel",
            Self::Refresh => "apps-refresh",
            Self::Launch => "apps-launch",
            Self::Cancel => "apps-cancel",
            Self::Pin => "apps-pin",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sort {
    #[default]
    Name,
    Reverse,
    Executable,
    Pinned,
}
impl Sort {
    pub fn label(self) -> &'static str {
        match self {
            Self::Name => "Name A–Z",
            Self::Reverse => "Name Z–A",
            Self::Executable => "Executable",
            Self::Pinned => "Pinned first",
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::Name => Self::Reverse,
            Self::Reverse => Self::Executable,
            Self::Executable => Self::Pinned,
            Self::Pinned => Self::Name,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Grouping {
    #[default]
    None,
    Pinned,
    Source,
}
impl Grouping {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "Ungrouped",
            Self::Pinned => "Pinned",
            Self::Source => "Source",
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::None => Self::Pinned,
            Self::Pinned => Self::Source,
            Self::Source => Self::None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Presentation {
    #[default]
    List,
    Grid,
}
impl Model {
    pub fn is_pinned(&self, entry: &Entry) -> bool {
        self.pins.pins.iter().any(|p| p.token == entry.token)
    }
    pub fn group<'a>(&self, entry: &'a Entry) -> &'a str {
        match self.grouping {
            Grouping::None => "",
            Grouping::Pinned => {
                if self.is_pinned(entry) {
                    "Pinned"
                } else {
                    "Other applications"
                }
            }
            Grouping::Source => {
                if entry.source.is_empty() {
                    "Source unavailable"
                } else {
                    entry.source
                }
            }
        }
    }
    fn group_key<'a>(&self, entry: &'a Entry) -> (u8, &'a str) {
        // Pin group first, rather than lexicographic 'Other applications'.
        (
            u8::from(self.grouping == Grouping::Pinned && !self.is_pinned(entry)),
            self.group(entry),
        )
    }
    pub(super) fn order(&mut self) {
        let Some(catalog) = &self.frame.catalog else {
            return;
        };
        let mut indices = std::mem::take(&mut self.filtered);
        indices.sort_by_cached_key(|&i| {
            let e = &catalog.entries[i];
            let name = e.name.to_lowercase();
            let filename = e
                .executable
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            (
                self.group_key(e).0,
                self.group_key(e).1.to_owned(),
                if self.sort == Sort::Pinned {
                    u8::from(!self.is_pinned(e))
                } else {
                    0
                },
                if self.sort == Sort::Executable {
                    filename
                } else {
                    String::new()
                },
                name,
                e.id.clone(),
            )
        });
        if self.sort == Sort::Reverse {
            // Reverse names within each group; keep group order stable.
            let mut start = 0;
            while start < indices.len() {
                let key = self.group_key(&catalog.entries[indices[start]]);
                let mut end = start + 1;
                while end < indices.len() && self.group_key(&catalog.entries[indices[end]]) == key {
                    end += 1;
                }
                indices[start..end].reverse();
                start = end;
            }
        }
        self.filtered = indices;
    }
    pub(super) fn order_missing(&mut self) {
        self.missing.sort_by_cached_key(|&i| {
            (
                self.pins.pins[i].name.to_lowercase(),
                self.pins.pins[i].token.clone(),
            )
        });
        if self.sort == Sort::Reverse {
            self.missing.reverse();
        }
    }
    pub fn apply_pins(&mut self, state: Arc<super::preferences::worker::State>) -> bool {
        if state.revision <= self.pins.revision {
            return false;
        }
        let changed = self.pins.pins != state.pins;
        self.pins = state;
        self.pins_pending = false;
        self.filter();
        changed
    }
    pub fn pin_enabled(&self) -> bool {
        self.pins.writable
            && !self.pins_pending
            && (self.selected().is_some() || self.missing_selected.is_some())
    }
    pub fn pin_change(&self) -> Result<Vec<Pin>, String> {
        if !self.pin_enabled() {
            return Err("Wait for pins to load or finish saving before changing favorites.".into());
        }
        let mut pins = (*self.pins.pins).clone();
        if let Some(token) = &self.missing_selected {
            pins.retain(|p| p.token != *token);
        } else if let Some(entry) = self.selected() {
            if self.is_pinned(entry) {
                pins.retain(|p| p.token != entry.token);
            } else {
                if entry.name.len() > super::preferences::MAX_LABEL {
                    return Err("This application's name is too long to save as a pin.".into());
                }
                pins.push(Pin {
                    token: entry.token.clone(),
                    name: entry.name.clone(),
                });
            }
        }
        super::preferences::encode(&pins)?;
        Ok(pins)
    }
    pub fn missing_pin(&self) -> Option<&Pin> {
        let token = self.missing_selected.as_ref()?;
        self.pins.pins.iter().find(|p| p.token == *token)
    }
    pub fn grid_columns(width: f32) -> usize {
        if width >= 1000.0 { 4 } else { 2 }
    }
    pub fn offset(&self, width: f32) -> f32 {
        let mut y = 0.0;
        let columns = if self.presentation == Presentation::Grid {
            Self::grid_columns(width)
        } else {
            1
        };
        let height = if self.presentation == Presentation::Grid {
            106.0
        } else {
            54.0
        };
        let mut group = "";
        let mut count = 0;
        if let Some(catalog) = &self.frame.catalog {
            for &i in &self.filtered {
                let entry = &catalog.entries[i];
                let next = self.group(entry);
                if next != group {
                    y += (count as f32 / columns as f32).ceil() * height;
                    count = 0;
                    y += 28.0;
                    group = next;
                }
                if self.selected.as_ref() == Some(&entry.id) {
                    return (y + (count / columns) as f32 * height - 50.0).max(0.0);
                }
                count += 1;
            }
        }
        y += (count as f32 / columns as f32).ceil() * height + 28.0;
        if self.missing_selected.is_some() {
            y += (self.position() - self.filtered.len()) as f32 * 54.0;
        }
        (y - 50.0).max(0.0)
    }
    pub fn focus_order(&self) -> Vec<Option<Focus>> {
        let mut order = vec![
            None,
            None,
            Some(Focus::Search),
            Some(Focus::Filter),
            Some(Focus::Presentation),
            Some(Focus::Sort),
            Some(Focus::Group),
            Some(Focus::Results),
        ];
        if !self.refreshing() {
            order.push(Some(Focus::Refresh));
        }
        if self.key().is_some() && !self.launching() {
            order.push(Some(Focus::Launch));
        }
        if self.pin_enabled() {
            order.push(Some(Focus::Pin));
        }
        if self.refreshing() {
            order.push(Some(Focus::Cancel));
        }
        order
    }
}
#[cfg(test)]
mod tests;

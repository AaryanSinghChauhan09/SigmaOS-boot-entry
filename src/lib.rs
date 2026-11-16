// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-boot-entry: boot entry component model.
//!
//! Real implementation: entrys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootEntryError {
    EmptyEntryName,
    DuplicateEntry(String),
    EmptyEventSubject,
}

impl fmt::Display for BootEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEntryName => write!(f, "entry name must not be empty"),
            Self::DuplicateEntry(id) => write!(f, "duplicate entry {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BootEntryError {}

pub struct BootEntry {
    pub name: String,
    entrys: BTreeMap<String, Entry>,
    events: BTreeMap<String, Event>,
}

impl BootEntry {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entrys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_entry(&mut self, item: Entry) -> Result<(), BootEntryError> {
        if item.name.trim().is_empty() {
            return Err(BootEntryError::EmptyEntryName);
        }
        let id = item.id.clone();
        if self.entrys.contains_key(&id) {
            return Err(BootEntryError::DuplicateEntry(id.clone()));
        }
        self.entrys.insert(id, item);
        Ok(())
    }

    pub fn entry_count(&self) -> usize {
        self.entrys.len()
    }

    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entrys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BootEntryError> {
        if event.subject.trim().is_empty() {
            return Err(BootEntryError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(id: &str, name: &str) -> Entry {
        Entry {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BootEntry::new("BootEntry");
        assert_eq!(m.name, "BootEntry");
        assert_eq!(m.entry_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_entry() {
        let mut m = BootEntry::new("BootEntry");
        m.add_entry(sample_entry("a1", "Alpha")).unwrap();
        assert_eq!(m.entry_count(), 1);
        assert!(m.get_entry("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BootEntry::new("BootEntry");
        assert!(m.add_entry(sample_entry("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BootEntry::new("BootEntry");
        m.add_entry(sample_entry("x", "One")).unwrap();
        assert!(m.add_entry(sample_entry("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BootEntry::new("BootEntry");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationTool {
    Select,
    Rectangle,
    Ellipse,
    Arrow,
    Line,
    Freehand,
    Text,
    Highlighter,
    Obfuscation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "rectangle")]
    Rectangle {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "ellipse")]
    Ellipse {
        id: String,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "arrow")]
    Arrow {
        id: String,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "line")]
    Line {
        id: String,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "freehand")]
    Freehand {
        id: String,
        points: Vec<(f64, f64)>,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "text")]
    Text {
        id: String,
        x: f64,
        y: f64,
        content: String,
        font_size: f64,
        color: String,
    },
    #[serde(rename = "highlighter")]
    Highlighter {
        id: String,
        points: Vec<(f64, f64)>,
        color: String,
        stroke_width: f64,
    },
    #[serde(rename = "obfuscation")]
    Obfuscation {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        block_size: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UndoRedoHistory<T> {
    pub undo_stack: Vec<T>,
    pub redo_stack: Vec<T>,
}

impl<T: Clone> UndoRedoHistory<T> {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Pushes a new operation and purges the redo stack
    pub fn push(&mut self, item: T) {
        self.undo_stack.push(item);
        self.redo_stack.clear();
    }

    /// Pops the last item and moves it to the redo stack
    pub fn undo(&mut self) -> Option<T> {
        if let Some(item) = self.undo_stack.pop() {
            self.redo_stack.push(item.clone());
            Some(item)
        } else {
            None
        }
    }

    /// Restores the last undone item back to the undo stack
    pub fn redo(&mut self) -> Option<T> {
        if let Some(item) = self.redo_stack.pop() {
            self.undo_stack.push(item.clone());
            Some(item)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn current_items(&self) -> &[T] {
        &self.undo_stack
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_undo_redo_history_lifecycle() {
        let mut history: UndoRedoHistory<String> = UndoRedoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        history.push("action1".to_string());
        history.push("action2".to_string());

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.current_items().len(), 2);

        // Undo action2
        let undone = history.undo().unwrap();
        assert_eq!(undone, "action2");
        assert_eq!(history.current_items().len(), 1);
        assert!(history.can_redo());

        // Redo action2
        let redone = history.redo().unwrap();
        assert_eq!(redone, "action2");
        assert_eq!(history.current_items().len(), 2);
        assert!(!history.can_redo());

        // Push purges redo stack
        history.undo();
        assert!(history.can_redo());
        history.push("action3".to_string());
        assert!(!history.can_redo());
        assert_eq!(history.current_items().len(), 2);
    }

    #[test]
    fn test_annotation_serialization() {
        let rect = Annotation::Rectangle {
            id: "rect-1".to_string(),
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 80.0,
            color: "#ef4444".to_string(),
            stroke_width: 4.0,
        };

        let json = serde_json::to_string(&rect).unwrap();
        assert!(json.contains("\"type\":\"rectangle\""));

        let deserialized: Annotation = serde_json::from_str(&json).unwrap();
        assert_eq!(rect, deserialized);
    }
}

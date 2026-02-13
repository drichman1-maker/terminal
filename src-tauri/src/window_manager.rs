use dashmap::DashMap;
use std::sync::Arc;

/// Window manager for Flex-Layout state and multi-window coordination
pub struct WindowManager {
    windows: Arc<DashMap<String, WindowState>>,
}

#[derive(Debug, Clone)]
pub struct WindowState {
    pub label: String,
    pub window_type: WindowType,
    pub layout: Option<String>, // JSON layout state
}

#[derive(Debug, Clone)]
pub enum WindowType {
    Main,
    OrderBook,
    Chart,
    News,
    Risk,
    Trade,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(DashMap::new()),
        }
    }

    pub fn register_window(&self, label: String, window_type: WindowType) {
        self.windows.insert(
            label.clone(),
            WindowState {
                label,
                window_type,
                layout: None,
            },
        );
    }

    pub fn save_layout(&self, label: &str, layout: String) {
        if let Some(mut window) = self.windows.get_mut(label) {
            window.layout = Some(layout);
        }
    }
}

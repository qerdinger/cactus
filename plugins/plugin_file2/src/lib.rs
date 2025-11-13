use plugin_api::*;
use ratatui::{prelude::*, widgets::*, Frame};

struct FilePane { cwd: std::path::PathBuf, entries: Vec<String> }

impl Default for FilePane {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or_default();
        let entries = std::fs::read_dir(&cwd)
            .unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
            .filter_map(|e| e.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect();
        Self { cwd, entries }
    }
}

impl Pane for FilePane {
    fn id(&self) -> &str { "file2.pane" }
    fn title(&self) -> &str { "Files2" }
    fn draw(&mut self, fw: &mut FrameWrapper) {
        let f: &mut Frame = unsafe { &mut *(fw.as_raw() as *mut Frame) };
        let area = f.size();
        let list = List::new(self.entries.iter().cloned().map(ListItem::new))
            .block(Block::default().title("Files2").borders(Borders::ALL));
        f.render_widget(list, area);
    }
}

struct FilePlugin;
impl Plugin for FilePlugin {
    fn id(&self) -> PluginId { "file2".into() }
    fn panes(&mut self) -> Vec<Box<dyn Pane>> { vec![Box::new(FilePane::default())] }
}

#[unsafe(no_mangle)]
pub extern "C" fn plugin_entry() -> Box<dyn Plugin> {
    Box::new(FilePlugin)
}

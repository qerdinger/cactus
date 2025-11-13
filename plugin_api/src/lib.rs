use std::time::Duration;

pub type PluginId = String;

pub trait AppBusTx: Send + Sync {
    fn publish(&self, topic: &str, payload: &[u8]);
}

pub trait AppBusRx: Send {
    fn try_recv(&mut self) -> Option<(String, Vec<u8>)>;
}

pub struct FrameWrapper<'a> {
    inner: *mut (),
    _phantom: std::marker::PhantomData<&'a mut ()>,
}
unsafe impl<'a> Send for FrameWrapper<'a> {}
impl<'a> FrameWrapper<'a> {
    pub fn new(ptr: *mut ()) -> Self {
        Self { inner: ptr, _phantom: std::marker::PhantomData }
    }
    pub fn as_raw(&mut self) -> *mut () { self.inner }
}

#[derive(Clone, Copy)]
pub enum InputEvent {
    Key { code: u32, ctrl: bool, alt: bool, shift: bool },
    Resize { w: u16, h: u16 },
}

pub enum Action {
    None,
    NeedsRedraw,
    OpenPane { id: String },
}

pub struct Tick {
    pub dt: Duration,
}

pub trait Pane: Send {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn draw(&mut self, f: &mut FrameWrapper);
    fn handle_input(&mut self, _e: InputEvent) -> Action { Action::None }
    fn on_tick(&mut self, _t: Tick) {}
}

pub trait Command: Send {
    fn name(&self) -> &str;
    fn help(&self) -> &str;
    fn run(&mut self, args: &[&str], bus: &dyn AppBusTx) -> anyhow::Result<()>;
}

pub trait Plugin: Send {
    fn api_version(&self) -> (u16, u16) { (0, 1) }
    fn id(&self) -> PluginId;
    fn init(&mut self, _bus: Box<dyn AppBusTx>) -> anyhow::Result<()> { Ok(()) }
    fn panes(&mut self) -> Vec<Box<dyn Pane>> { Vec::new() }
    fn commands(&mut self) -> Vec<Box<dyn Command>> { Vec::new() }
    fn start_tasks(&mut self, _bus: Box<dyn AppBusTx>) -> anyhow::Result<()> { Ok(()) }
}

pub const ENTRY_SYMBOL: &[u8] = b"plugin_entry";
pub type EntryFn = unsafe extern "C" fn() -> Box<dyn Plugin>;

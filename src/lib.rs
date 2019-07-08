pub trait Events {
    fn on_init(&self) {}
    fn on_error(&self, _err: &str) {}
    fn on_dispatch(&mut self, _s: &String) {}
    fn on_input(&mut self, _s: &String) {}
    fn on_shutdown(&self) {}
    fn on_pre_read(&self) {}
    fn on_post_read(&self) {}
}

pub struct PluginManager {
    hooks: Vec<Box<dyn Events>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
    pub fn add_events_hook<E: Events + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }

    pub fn dispatch(&mut self, s: String) {
        for hook in &mut self.hooks {
            hook.on_dispatch(&s);
        }
    }
}

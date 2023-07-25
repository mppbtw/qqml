pub struct MultichoicePanel {
    options: Vec<MultichoiceOption>,
}
impl MultichoicePanel {
    pub fn add_option(&mut self, option: MultichoiceOption) {
        self.options.push(option);
    }

    pub fn display(&self) {}
}

pub struct MultichoiceOption {
    listener: Option<Box<dyn Fn()>>,
    lable: Option<String>,
}

impl MultichoiceOption {
    pub fn bind_listener(&mut self, listener: Box<dyn Fn()>) {
        if self.listener.is_none() {
            self.listener = Some(listener);
        }
    }

    pub fn set_lable(&mut self, lable: String) {
        if self.lable.is_none() {
            self.lable = Some(lable);
        }
    }
}

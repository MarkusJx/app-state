#[macro_export]
macro_rules! create_state {
    ($init: ident) => {
        struct State {
            name: String,
        }

        impl StateTrait for State {
            fn get_name(&self) -> &str {
                self.name.as_str()
            }

            fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
        }

        $init::init(State {
            name: "Hello".to_string(),
        });
    };
}

pub(crate) trait StateTrait: Send + Sync + 'static {
    fn get_name(&self) -> &str;

    fn set_name(&mut self, name: &str);
}

pub mod deno;
pub mod headless;
pub mod node;
pub mod server;
pub mod shell;

pub use walrus;

pub struct Tests {
    pub tests: Vec<Test>,
    pub filtered: usize,
}

impl Tests {
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            filtered: 0,
        }
    }

    fn as_args(&self, include_ignored: bool) -> String {
        let filtered = self.filtered;

        format!(
            r#"
            // Forward runtime arguments.
            cx.include_ignored({include_ignored:?});
            cx.filtered_count({filtered});
        "#
        )
    }
}

pub struct Test {
    /// The test name.
    pub name: String,

    /// Symbol name.
    pub export: String,

    /// Whether or not the test should be ignored.
    pub ignored: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TestMode {
    Node { no_modules: bool },
    Deno,
    Browser { no_modules: bool },
    DedicatedWorker { no_modules: bool },
    SharedWorker { no_modules: bool },
    ServiceWorker { no_modules: bool },
}

impl TestMode {
    pub fn is_worker(self) -> bool {
        matches!(
            self,
            Self::DedicatedWorker { .. } | Self::SharedWorker { .. } | Self::ServiceWorker { .. }
        )
    }

    pub fn no_modules(self) -> bool {
        match self {
            Self::Deno => true,
            Self::Browser { no_modules }
            | Self::Node { no_modules }
            | Self::DedicatedWorker { no_modules }
            | Self::SharedWorker { no_modules }
            | Self::ServiceWorker { no_modules } => no_modules,
        }
    }
}

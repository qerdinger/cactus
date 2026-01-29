use crate::discovery::lang::Lang;
use crate::interpreter::lang_interpreter::LangInterpreter;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

struct InterpreterEntry {
    interpreter: Arc<dyn LangInterpreter + Send + Sync>,
    available: bool,
}

pub struct InterpreterEngine {
    idx: usize,
    interpreters: HashMap<usize, InterpreterEntry>,
}

impl InterpreterEngine {
    pub fn new() -> Self {
        Self {
            idx: 0,
            interpreters: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, interpreter: T)
    where
        T: LangInterpreter + Send + Sync + 'static,
    {
        let idx = self.idx;

        self.interpreters.insert(
            idx,
            InterpreterEntry {
                interpreter: Arc::new(interpreter),
                available: true,
            },
        );

        self.idx += 1;
    }

    pub fn with_interpreter_for_lang<R, F>(&mut self, lang: &Lang, f: F) -> Option<R>
    where
        F: FnOnce(&dyn LangInterpreter) -> R,
    {
        let idx = self
            .interpreters
            .iter()
            .find(|(_, entry)| entry.available && entry.interpreter.lang() == lang)
            .map(|(idx, _)| *idx)?;

        self.interpreters.get_mut(&idx)?.available = false;

        let result = {
            let entry = self.interpreters.get(&idx)?;
            f(entry.interpreter.as_ref())
        };

        self.interpreters.get_mut(&idx)?.available = true;

        Some(result)
    }
}

impl fmt::Display for InterpreterEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outs = self
            .interpreters
            .iter()
            .map(|(idx, entry)| format!("{}: [{}]", idx, entry.interpreter.lang()))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", outs)
    }
}

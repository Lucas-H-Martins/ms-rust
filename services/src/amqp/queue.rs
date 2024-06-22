#[derive(Debug, Clone, Default)]
pub struct QueueDefinition {
    pub(crate) name: String,
    pub(crate) durable: bool,
    pub(crate) delete: bool,
    pub(crate) exclusive: bool,
    pub(crate) passive: bool,
    pub(crate) no_wait: bool,
    pub(crate) ttl: Option<i32>,
    pub(crate) dlq_name: Option<String>,
    pub(crate) retry_name: Option<String>,
    pub(crate) retry_ttl: Option<i32>,
    pub(crate) retries: Option<i32>,
}

impl QueueDefinition {
    pub fn new(name: &str) -> QueueDefinition {
        QueueDefinition {
            name: name.to_owned(),
            durable: false,
            delete: false,
            exclusive: false,
            passive: false,
            no_wait: false,
            ttl: None,
            dlq_name: None,
            retry_name: None,
            retry_ttl: None,
            retries: None,
        }
    }

    pub fn durable(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn delete(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn exclusive(mut self) -> Self {
        self.durable = true;
        self
    }
    pub fn passive(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn no_wait(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn ttl(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn dlq_name(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn retry_ttl(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn retries(mut self) -> Self {
        self.durable = true;
        self
    }
}

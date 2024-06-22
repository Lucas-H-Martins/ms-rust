use super::amqp::AmqpConfigs;

#[derive(Debug, Clone, Default)]
pub struct Configs<T: VariableConfigs> {
    pub amqp: AmqpConfigs,
}

pub trait VariableConfigs: Default {
    fn load(&mut self);
}

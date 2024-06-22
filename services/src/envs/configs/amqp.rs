pub struct AmqpConfigs {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub pass: String,
}
impl Default for AmqpConfigs {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: 5671.to_owned(),
            user: "guest".to_owned(),
            pass: "guest".to_owned(),
        }
    }
}

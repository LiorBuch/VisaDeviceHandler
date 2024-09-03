use mutex_logger::logger::Verbosity;

pub struct MapConfig {
    pub panic_verbosity:Verbosity
}
impl MapConfig {
    pub fn default() ->MapConfig {
        return MapConfig{panic_verbosity:Verbosity::Error};
    }
}
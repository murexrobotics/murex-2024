use core::default::Default;

pub struct Config {
    pub enabled: bool,
    pub active_mode: bool,
    pub config_after_restart: bool,
    pub config_after_sleep: bool,
    pub fifo_enabled: bool,
    pub int1: IntConfig,
    pub int2: IntConfig,
    pub self_test: Option<bool>,
    pub osr: OsrLevel,
    pub odr: OdrLevel,
    pub range: Range,
}

impl Default for Config {
    fn default() -> Self {
        // TODO: Verify Default
        Self {
            enabled: true,
            active_mode: true,
            config_after_restart: true,
            config_after_sleep: true,
            fifo_enabled: false,
            int1: IntConfig {
                enabled: false,
                input: false,
                output: false,
                map_data_ready: false,
                map_fifo_watermark: false,
                map_fifo_full: false,
                stream_mode: false,
                store_int_data: false,
                push_pull: false,
                active_high: false,
            },
            int2: IntConfig {
                enabled: false,
                input: false,
                output: false,
                map_data_ready: false,
                map_fifo_watermark: false,
                map_fifo_full: false,
                stream_mode: false,
                store_int_data: false,
                push_pull: false,
                active_high: false,
            },
            self_test: None,
            osr: OsrLevel::Normal,
            odr: OdrLevel::Odr100,
            range: Range::G3,
        }
    }
}

impl Config {
    fn murex_config() -> Self {
        todo!();
    }
}

pub struct IntConfig {
    pub enabled: bool,
    pub input: bool,
    pub output: bool,
    pub map_data_ready: bool,
    pub map_fifo_watermark: bool,
    pub map_fifo_full: bool,
    pub stream_mode: bool, // VS FIFO Mode
    pub store_int_data: bool,
    pub push_pull: bool,   // VS Open Drain
    pub active_high: bool, // VS Active Low
}

pub enum OsrLevel {
    Osr4,
    Osr2,
    Normal,
}

pub enum OdrLevel {
    Odr12_5,
    Odr25,
    Odr50,
    Odr100,
    Odr200,
    Odr400,
    Odr800,
    Odr1600,
}

pub enum Range {
    G3,
    G6,
    G12,
    G24,
}

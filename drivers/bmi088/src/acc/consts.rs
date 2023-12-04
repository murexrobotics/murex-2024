// --------------- Registers ------------------
pub(crate) const ACC_SOFTRESET: u8 = 0x7E;
pub(crate) const ACC_PWR_CTRL: u8 = 0x7D;
pub(crate) const ACC_PWR_CONF: u8 = 0x7C;
pub(crate) const ACC_SELF_TEST: u8 = 0x6D;
pub(crate) const INT_MAP_DATA: u8 = 0x58; // Interesting, this has 2 name check page 25 and 33
pub(crate) const INT2_IO_CTRL: u8 = 0x54;
pub(crate) const INT1_IO_CTRL: u8 = 0x53;
pub(crate) const FIFO_CONFIG_1: u8 = 0x49;
pub(crate) const FIFO_CONFIG_0: u8 = 0x48;
pub(crate) const FIFO_WTM_1: u8 = 0x47;
pub(crate) const FIFO_WTM_0: u8 = 0x46;
pub(crate) const FIFO_DOWNS: u8 = 0x45;
pub(crate) const ACC_RANGE: u8 = 0x41;
pub(crate) const ACC_CONF: u8 = 0x40;
pub(crate) const FIFO_DATA: u8 = 0x26;
pub(crate) const FIFO_LENGTH_1: u8 = 0x25;
pub(crate) const FIFO_LENGTH_0: u8 = 0x24;
pub(crate) const TEMP_LSB: u8 = 0x23;
pub(crate) const TEMP_MSB: u8 = 0x22;
pub(crate) const ACC_INT_STAT_1: u8 = 0x1D;
pub(crate) const SENSORTIME_2: u8 = 0x1A;
pub(crate) const SENSORTIME_1: u8 = 0x19;
pub(crate) const SENSORTIME_0: u8 = 0x18;
pub(crate) const ACC_Z_MSB: u8 = 0x17;
pub(crate) const ACC_Z_LSB: u8 = 0x16;
pub(crate) const ACC_Y_MSB: u8 = 0x15;
pub(crate) const ACC_Y_LSB: u8 = 0x14;
pub(crate) const ACC_X_MSB: u8 = 0x13;
pub(crate) const ACC_X_LSB: u8 = 0x12;
pub(crate) const ACC_STATUS: u8 = 0x03;
pub(crate) const ACC_ERR_REG: u8 = 0x02;
pub(crate) const ACC_CHIP_ID: u8 = 0x00;

// ----------- Resgiter Reserve Masks ------------
// These masks are intended to be used to preserve
// reserved bits in registers. Use when writing to
// RW registers
pub(crate) const INT_MAP_DATA_RESERVED: u8 = 0b10001000;
pub(crate) const INT2_IO_CTRL_RESERVED: u8 = 0b11100001;
pub(crate) const INT1_IO_CTRL_RESERVED: u8 = 0b11100001;
pub(crate) const FIFO_CONFIG_1_RESERVED: u8 = 0b10110011; // bit 4 is always 1
pub(crate) const FIFO_CONFIG_0_RESERVED: u8 = 0b11111110; // bit 1 is always 1
pub(crate) const FIFO_WTM_1_RESERVED: u8 = 0b11100000;
pub(crate) const FIFO_DOWNS_RESERVED: u8 = 0b10011111; // bit 7 is always 1
pub(crate) const ACC_RANGE_RESERVED: u8 = 0b11111100;

// ------------------ RO Masks -------------------
pub(crate) const ACC_DATA_READY_INT1: u8 = 0b10000000; // ACC_INT_STAT_1
pub(crate) const ACC_DATA_READY: u8 = 0b10000000; // ACC_STATUS
pub(crate) const ACC_NO_ERR: u8 = 0b00000000; // ACC_ERR_REG
pub(crate) const ACC_FATAL_ERR: u8 = 0b00000001; // ACC_ERR_REG

// ------------------ RW Masks -------------------

// ACC_PWR_CONF
pub(crate) const ACC_ACTIVE_MODE: u8 = 0x00;
pub(crate) const ACC_SUSPEND_MODE: u8 = 0x03;

// ACC_PWR_CTRL
pub(crate) const ACC_ON: u8 = 0x04;
pub(crate) const ACC_OFF: u8 = 0x00;

// ACC_SOFTRESET - WRITE ONLY
pub(crate) const ACC_SOFTRESET_CMD: u8 = 0xb6;

// ACC_CONF
pub(crate) const ACC_OSR4: u8 = 0b10000000;
pub(crate) const ACC_OSR2: u8 = 0b10010000;
pub(crate) const ACC_OSR_NORMAL: u8 = 0b10100000;
pub(crate) const ACC_ODR12_5: u8 = 0x05;
pub(crate) const ACC_ODR25: u8 = 0x06;
pub(crate) const ACC_ODR50: u8 = 0x07;
pub(crate) const ACC_ODR100: u8 = 0x08;
pub(crate) const ACC_ODR200: u8 = 0x09;
pub(crate) const ACC_ODR400: u8 = 0x0A;
pub(crate) const ACC_ODR800: u8 = 0x0B;
pub(crate) const ACC_ODR1600: u8 = 0x0C;

// ACC_RANGE
pub(crate) const ACC_RANGE_3G: u8 = 0x00;
pub(crate) const ACC_RANGE_6G: u8 = 0x01;
pub(crate) const ACC_RANGE_12G: u8 = 0x02;
pub(crate) const ACC_RANGE_24G: u8 = 0x03;

// ACC_SELF_TEST
pub(crate) const ACC_TEST_OFF: u8 = 0x00;
pub(crate) const ACC_TEST_POSITIVE: u8 = 0x0D;
pub(crate) const ACC_TEST_NEGATIVE: u8 = 0x09;

// INT_MAP_DATA
pub(crate) const INT2_DATA_READY_MAPPING: u8 = 0b01000000;
pub(crate) const INT2_FIFO_WATERMARK_INT_MAPPING: u8 = 0b00100000;
pub(crate) const INT2_FIFO_FULL_INT_MAPPING: u8 = 0b00010000;
pub(crate) const INT1_DATA_READY_MAPPING: u8 = 0b00000100;
pub(crate) const INT1_FIFO_WATERMARK_INT_MAPPING: u8 = 0b00000010;
pub(crate) const INT1_FIFO_FULL_INT_MAPPING: u8 = 0b00000001;

// INT1_IO_CTRL
pub(crate) const INT1_INPUT_EN: u8 = 0b00010000;
pub(crate) const INT1_OUTPUT_EN: u8 = 0b00001000;
pub(crate) const INT1_PUSH_PULL: u8 = 0b00000000;
pub(crate) const INT1_OPEN_DRAIN: u8 = 0b00000100;
pub(crate) const INT1_ACTIVE_HIGH: u8 = 0b00000010;
pub(crate) const INT1_ACTIVE_LOW: u8 = 0b00000000;

// INT2_IO_CTRL
pub(crate) const INT2_INPUT_EN: u8 = 0b00010000;
pub(crate) const INT2_OUTPUT_EN: u8 = 0b00001000;
pub(crate) const INT2_PUSH_PULL: u8 = 0b00000000;
pub(crate) const INT2_OPEN_DRAIN: u8 = 0b00000100;
pub(crate) const INT2_ACTIVE_HIGH: u8 = 0b00000010;
pub(crate) const INT2_ACTIVE_LOW: u8 = 0b00000000;

// FIFO_DOWNS
// TODO: Remove
pub(crate) const FIFO_DOWNSAMPLING: u8 = 0x00; // Interesting, in table it is listed as RW, but description is RO

// FIFO_CONFIG_0
pub(crate) const ACC_FIFO_STREAM: u8 = 0x00;
pub(crate) const ACC_FIFO_MODE: u8 = 0x01;

// FIFO_CONFIG_1
pub(crate) const ENABLE_ACC_FIFO: u8 = 0b01000000;
pub(crate) const ENABLE_FIFO_INT1_STORE: u8 = 0b00001000;
pub(crate) const ENABLE_FIFO_INT2_STORE: u8 = 0b00000100;

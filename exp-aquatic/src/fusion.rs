use crate::telemetry::{self, TelemetryPacket};

use ahrs::{Ahrs, Madgwick};
use nalgebra::{Quaternion, Vector3};

/// Madgwick filter sample-rate, adjust based on duration of telemetry cycle
const SAMPLE_RATE: f32 = 0.1;
/// Madgwick filter gain, adjust experimentally
const BETA: f32 = 0.1;

/// Initializes sensor fusion algorithms.
/// 
/// ## Returns:
/// Callable functions that will calculate information about the state of the system based on collected telemetry.
/// 
/// ## Example Usage:
/// ```Rust
/// use crate::fusion;
/// 
/// let (ahrs, pos) = fusion::init_sensor_fusion();
/// ... // Obtain telemetry in the form of `TelemetryPacket`
/// let (pitch, roll, yaw) = ahrs(telemetry);
/// let (x, y, z) = pos(telemetry);
/// ```
pub fn init_sensor_fusion() -> (
    impl FnMut(&TelemetryPacket) -> (f32, f32, f32),
    impl FnMut(&TelemetryPacket) -> (f32, f32, f32),
) {
    let mut ahrs = Madgwick::new(SAMPLE_RATE, BETA);

    return (
        move |telemetry| calculate_ahrs(&mut ahrs, telemetry),
        move |telemetry| todo!(),
    );
}

/// Attitude and Heading *Something* System. Calculates absolute based 
/// on measured changes to magnetic field strength, linear acceleration 
/// and angular velocity.
///
/// ## Arguments:
/// *   `ahrs`: Madgwick filter with previous state calculations.
/// *   `telemetry`: Raw telemetry sensor measurements
///
/// ## Returns:
/// Tuple representing rotation in radians (pitch, roll, yaw)
fn calculate_ahrs(ahrs: &mut Madgwick<f32>, telemetry: &TelemetryPacket) -> (f32, f32, f32) {
    let accel = Vector3::new(
        telemetry.acceleration.0,
        telemetry.acceleration.1,
        telemetry.acceleration.2,
    );
    // Gyro data needs to be in radians, fix if necessarry
    let gyro = Vector3::new(
        telemetry.angular_velocity.0,
        telemetry.angular_velocity.1,
        telemetry.angular_velocity.2,
    );
    // Magnetometer data is in gauss, TODO: convert to right units, unsure if it is Tesla's or not
    let mag = Vector3::new(
        telemetry.magnetic_field.0 as f32,
        telemetry.magnetic_field.1 as f32,
        telemetry.magnetic_field.2 as f32,
    );

    let quat = ahrs
        .update(&gyro, &accel, &mag)
        .expect("Failed to update AHRS filter");

    let (roll, pitch, yaw) = quat.euler_angles();
    (roll, pitch, yaw)
}

/// Position Estimation System.
fn calculate_position(telemetry: &TelemetryPacket) -> (f32, f32, f32) {
    todo!()
}

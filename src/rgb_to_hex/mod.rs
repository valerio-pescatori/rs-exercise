/// The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.
/// Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.
///
/// Examples (input --> output):
///
/// ```rs
///     255, 255, 255 --> "FFFFFF"
///     255, 255, 300 --> "FFFFFF"
///     0, 0, 0       --> "000000"
///     148, 0, 211   --> "9400D3"
/// ```
pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let c_r = r.clamp(0, 255);
    let c_g = g.clamp(0, 255);
    let c_b = b.clamp(0, 255);

    format!("{:02X}{:02X}{:02X}", c_r, c_g, c_b)
}

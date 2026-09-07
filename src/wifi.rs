use std::process::Command;

fn set_wifi(ssid: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("nmcli")
        .args(["device", "wifi", "connect", ssid, "password", password])
        .status()?;

    if !status.success() {
        return Err("Failed to connect to Wi-Fi".into());
    }

    Ok(())
}

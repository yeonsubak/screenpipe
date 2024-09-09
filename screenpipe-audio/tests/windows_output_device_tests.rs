#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use screenpipe_audio::{list_audio_devices, record_and_transcribe};
    use tokio::sync::mpsc;

    use super::*;
    use std::{
        path::PathBuf,
        sync::{atomic::AtomicBool, Arc},
        time::Duration,
    };

    #[tokio::test]
    async fn test_virtual_audio_device() -> anyhow::Result<()> {
        // List devices and find the virtual output device
        let devices = list_audio_devices().await?;
        let virtual_device = devices
            .iter()
            .find(|d| d.name.contains("CABLE Output"))
            .unwrap();

        println!("Found virtual device: {:?}", virtual_device.name);

        // Set up for recording
        let duration = Duration::from_secs(5);
        let output_path = PathBuf::from("test_output.mp4");
        let (tx, _rx) = mpsc::unbounded_channel();
        let is_running = Arc::new(AtomicBool::new(true));

        // Record from the virtual device
        let result = record_and_transcribe(
            Arc::new(virtual_device.clone()),
            duration,
            output_path.clone(),
            tx,
            is_running.clone(),
        )
        .await;

        assert!(result.is_ok());
        assert!(output_path.exists());

        println!("result: {:?}", result.unwrap());

        // You might want to add more assertions here, like checking file size, etc.

        Ok(())
    }
}
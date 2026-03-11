use clap::{Parser, Subcommand};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use reqwest::multipart;

#[derive(Parser)]
#[command(name = "workflow")]
#[command(about = "Workflow CLI - Record and manage your workflow", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Record audio immediately
    #[arg(short, long)]
    record: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Record audio and upload to the workflow server
    Record,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.record || matches!(cli.command, Some(Commands::Record)) {
        record_and_upload().await?;
    } else {
        println!("Usage: workflow --record");
    }

    Ok(())
}

async fn record_and_upload() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to find a default input device");

    let config: cpal::StreamConfig = device.default_input_config()?.into();
    println!("Recording from device: {}", device.name()?);
    println!("Recording with config: {:?}", config);

    let spec = hound::WavSpec {
        channels: config.channels,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = Arc::new(Mutex::new(Some(hound::WavWriter::create("temp_recording.wav", spec)?)));
    let writer_clone = writer.clone();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Some(ref mut w) = *writer_clone.lock().unwrap() {
                for &sample in data {
                    let amplitude = (sample * i16::MAX as f32) as i16;
                    w.write_sample(amplitude).ok();
                }
            }
        },
        |err| eprintln!("An error occurred on the audio stream: {}", err),
        None,
    )?;

    stream.play()?;
    println!("Recording... Press Ctrl+C to stop.");

    // Wait for Ctrl+C
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        let _ = tx.send(());
    });

    rx.await?;

    println!("\nStopping recording...");
    stream.pause()?;
    
    // Explicitly drop the writer to ensure the WAV header is correctly written
    {
        let mut w = writer.lock().unwrap();
        w.take(); 
    }

    println!("Uploading to server...");
    upload_file("temp_recording.wav").await?;

    // Cleanup
    std::fs::remove_file("temp_recording.wav")?;
    println!("Recording uploaded successfully!");

    Ok(())
}

async fn upload_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let bytes = tokio::fs::read(path).await?;
    
    let part = multipart::Part::bytes(bytes)
        .file_name("recording.wav")
        .mime_str("audio/wav")?;

    let form = multipart::Form::new().part("file", part);

    let url = "http://localhost:4000/upload";
    let res = client.post(url).multipart(form).send().await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let status = res.status();
        let err_text = res.text().await?;
        Err(format!("Upload failed: {} - {}", status, err_text).into())
    }
}

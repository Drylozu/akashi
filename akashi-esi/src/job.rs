use std::env::temp_dir;
use std::path::PathBuf;
use std::process::Command;
use akashi_shared::{AkashiContext, AkashiErr};
use poise::CreateReply;
use poise::serenity_prelude::CreateAttachment;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::env::var;

#[cfg(not(debug_assertions))]
pub const SIC_EXECUTABLE: &str = var("COMMAND_RELEASE_PATH").expect("SIC_EXECUTABLE missing").parse::<&str>().unwrap();
#[cfg(debug_assertions)]
pub const SIC_EXECUTABLE: &str = var("COMMAND_DEBUG_PATH").expect("SIC_EXECUTABLE missing").parse::<&str>().unwrap();

pub const MAX_MB: u8 = var("MAX_SOURCE_SIZE").expect("MAX_SOURCE_SIZE missing").parse::<u8>().unwrap();

pub struct SicJob {
    pub temp_dir: PathBuf,
    pub file_path: PathBuf,
    pub name: String,
    pub format: String,
    pub cmd: Command,
    start: std::time::Instant
}

impl SicJob {
    pub fn new(format: String) -> Self {
        let name = format!("output.{}", format);
        let temp_dir = temp_dir();
        let file_path = temp_dir.join(&name);

        Self {
            temp_dir,
            file_path,
            name,
            format,
            cmd: Command::new(SIC_EXECUTABLE),
            start: std::time::Instant::now()
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    pub fn args(&mut self, args: &[&str]) -> &mut Self {
        self.cmd.args(args);
        self
    }

    pub async fn with_bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        println!("{}", bytes.len());

        if bytes.len() > (MAX_MB as usize) * 1024 * 1024 {
            return self;
        }

        let mut file = File::create(&self.file_path).await.unwrap();
        file.write_all(&bytes).await.unwrap();
        file.flush().await.unwrap();

        let file_path = self.file_path.display().to_string();
        let format = self.format.clone();

        self.cmd.args(&[
            "-i",
            &file_path,
            "-o",
            &file_path,
            "--output-format",
            &format,
        ]);

        self
    }

    /// Applies a gaussian blur mask
    pub fn blur(&mut self, sigma: f32) -> &mut Self {
        self.cmd.args(&["--blur", &sigma.to_string()]);
        self
    }

    /// Speech balloon
    pub fn speech(&mut self) -> &mut Self {
        self.cmd.args(&["--speech", "./akashi-esi/assets/speech.png"]);
        self
    }

    pub async fn run(&mut self, ctx: AkashiContext<'_>) -> Result<(), AkashiErr> {
        let exit_status = self.cmd.spawn()?.wait()?;

        if exit_status.success() {
            let output_file = File::open(&self.file_path).await.map_err(|e| {
                AkashiErr::from(format!("Error opening output file: {e:?}"))
            })?;
            let attachment = CreateAttachment::file(&output_file, &self.name).await?;

            ctx.send(
                CreateReply::default()
                    .content(format!("Took {:.2}ms", self.start.elapsed().as_millis()))
                    .attachment(attachment),
            )
                .await?;

            self.temp_dir.clear();
            Ok(())
        } else {
            Err(AkashiErr::from(format!(
                "Command exited with non-zero status code: {}",
                exit_status
            )))
        }
    }
}
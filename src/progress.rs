// Imports for Indicatif (Progress bar lib for CLI)
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Mutex;
/** This struct represents the progress bar, but in an async style to make it suitable for multithreading used in main.rs
*/
#[derive(Debug, Clone)]
pub struct Progress {
    bar: Arc<Mutex<ProgressBar>>,
}

impl Progress {
    /** Creates a new Progress bar with predetermined properties (can be seen in .template(..))
    */
    pub fn new(total: usize) -> Self {
        let bar = ProgressBar::new(total as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        Self {
            bar: Arc::new(Mutex::new(bar)),
        }
    }

    pub async fn inc(&self) {
        let bar = self.bar.lock().await;
        bar.inc(1);
    }

    pub async fn finish(&self) {
        let bar = self.bar.lock().await;
        bar.finish_with_message("Finished Searching!")
    }
}

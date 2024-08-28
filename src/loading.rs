use std::time::Duration;
use indicatif::ProgressBar;
use anyhow::Result;

pub async fn perform_with_loading<F, Fut, T>(message: &str, task: F) -> Result<T>
where
    F: FnOnce() -> Fut + Send,
    Fut: std::future::Future<Output = Result<T>> + Send,
{
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(message.to_string());

    let result = task().await;

    let final_message = format!("{} done.", message.trim_end_matches("..."));
    pb.finish_with_message(final_message);

    result
}

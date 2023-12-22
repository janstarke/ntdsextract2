use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

#[allow(dead_code)]
pub(crate) fn create_progressbar(message: String, count: u64) -> anyhow::Result<ProgressBar> {
    let bar = indicatif::ProgressBar::new(count);
    let target = ProgressDrawTarget::stderr_with_hz(10);
    bar.set_draw_target(target);

    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9}({percent}%) {msg}")?
        .progress_chars("##-");
    bar.set_style(progress_style);
    bar.set_message(message);
    Ok(bar)
}

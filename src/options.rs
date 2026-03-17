use super::*;

#[derive(Debug, Clone, Parser)]
pub(crate) struct Options {
  #[clap(
    short,
    long,
    value_name = "COMMAND",
    value_delimiter = ',',
    help = "Filter panes by command (comma-separated). Default: claude"
  )]
  pub(crate) commands: Vec<String>,
  #[clap(
    short = 's',
    long,
    value_name = "SESSION",
    value_delimiter = ',',
    help = "Filter panes by session name (comma-separated). Auto-detects current session when inside tmux"
  )]
  pub(crate) sessions: Vec<String>,
  #[clap(long, help = "Show only panes running Claude Code (default: enabled)")]
  pub(crate) claude: bool,
  #[clap(short, long, help = "Disable colored output")]
  pub(crate) no_colors: bool,
  #[clap(
    long = "refresh-rate",
    value_name = "MILLISECONDS",
    value_parser = clap::value_parser!(NonZeroU64),
    help = "Refresh interval in milliseconds (default: 500)"
  )]
  pub(crate) refresh_rate: Option<NonZeroU64>,
}

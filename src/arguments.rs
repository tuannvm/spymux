use super::*;

#[derive(Debug, Clone, Parser)]
#[clap(
  about,
  author,
  version,
  help_template = "\
{before-help}{name} {version}

{about}

\x1b[1;4mUsage\x1b[0m: {usage}

{all-args}{after-help}
"
)]
pub(crate) struct Arguments {
  #[clap(flatten)]
  pub(crate) options: Options,
  #[clap(subcommand)]
  pub(crate) subcommand: Option<Subcommand>,
}

impl Arguments {
  fn get_current_tmux_session() -> Option<String> {
    // Only proceed if we're inside tmux
    if env::var("TMUX").is_err() {
      return None;
    }

    // Get current session name from tmux
    Command::new("tmux")
      .args(["display-message", "-p", "#{session_name}"])
      .output()
      .ok()
      .and_then(|output| {
        // Validate tmux command succeeded
        if !output.status.success() {
          return None;
        }
        let session = String::from_utf8(output.stdout).ok()?;
        let session = session.trim();
        if session.is_empty() {
          None
        } else {
          Some(session.to_string())
        }
      })
  }

  pub(crate) fn run(self) -> Result {
    if let Some(subcommand) = self.subcommand {
      subcommand.run()
    } else {
      let refresh_rate = self.options.refresh_rate.map_or_else(
        || Config::default().refresh_rate,
        |rate| Duration::from_millis(rate.get()),
      );

      // Handle --claude flag: adds "claude" to command filter
      let mut command_filter = self.options.commands;
      if self.options.claude && !command_filter.iter().any(|c| c.eq_ignore_ascii_case("claude")) {
        command_filter.push("claude".to_string());
      }

      // Default to claude filter if no command filter specified
      let command_filter = if command_filter.is_empty() {
        vec!["claude".to_string()]
      } else {
        command_filter
      };

      // Auto-detect current tmux session if inside tmux and no explicit session filter
      let session_filter = if self.options.sessions.is_empty() {
        Self::get_current_tmux_session()
          .map(|s| vec![s])
          .unwrap_or_default()
      } else {
        self.options.sessions
      };

      App::new(Config {
        color_output: !self.options.no_colors,
        command_filter,
        session_filter,
        refresh_rate,
      })?
      .run()
    }
  }
}

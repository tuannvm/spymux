use super::*;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub(crate) struct Pane {
  pub(crate) command: String,
  #[serde(default)]
  pub(crate) content: String,
  pub(crate) id: String,
  pub(crate) index: usize,
  pub(crate) path: String,
  pub(crate) session: String,
  #[serde(default)]
  pub(crate) window_name: String,
  pub(crate) window_index: usize,
}

impl Pane {
  pub(crate) fn descriptor(&self) -> String {
    format!("{}:{}.{}", self.session, self.window_index, self.index)
  }

  pub(crate) fn format<'a>() -> &'a str {
    concat!(
      "{",
      "\"command\":\"#{pane_current_command}\",",
      "\"id\":\"#{pane_id}\",",
      "\"index\":#{pane_index},",
      "\"path\":\"#{pane_current_path}\",",
      "\"session\":\"#{session_name}\",",
      "\"window_index\":#{window_index},",
      "\"window_name\":\"#{window_name}\"",
      "}"
    )
  }

  pub(crate) fn title(&self) -> String {
    let command = self.command.trim();
    let window_name = self.window_name.trim();

    let descriptor = if window_name.is_empty() {
      format!("{}:{}", self.session, self.window_index)
    } else {
      format!("{}:{}:{}", self.session, self.window_index, window_name)
    };

    if command.is_empty() {
      return descriptor;
    }

    format!("{} ({command})", descriptor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn descriptor_displays_session_window_and_index() {
    let pane = Pane {
      index: 1,
      session: "session".into(),
      window_index: 2,
      ..Default::default()
    };

    assert_eq!(pane.descriptor(), "session:2.1");
  }

  #[test]
  fn title_appends_command_when_present() {
    let pane = Pane {
      command: "bash".into(),
      index: 1,
      session: "session".into(),
      window_index: 2,
      ..Default::default()
    };

    assert_eq!(pane.title(), "session:2 (bash)");
  }

  #[test]
  fn title_displays_window_name_and_index_when_present() {
    let pane = Pane {
      command: "bash".into(),
      index: 1,
      session: "session".into(),
      window_name: "my-window".into(),
      window_index: 2,
      ..Default::default()
    };

    assert_eq!(pane.title(), "session:2:my-window (bash)");
  }

  #[test]
  fn title_falls_back_to_window_index_when_no_name() {
    let pane = Pane {
      command: "bash".into(),
      index: 1,
      session: "session".into(),
      window_index: 2,
      ..Default::default()
    };

    assert_eq!(pane.title(), "session:2 (bash)");
  }

  #[test]
  fn title_omits_command_when_blank() {
    let pane = Pane {
      index: 1,
      session: "session".into(),
      window_name: "my-window".into(),
      window_index: 2,
      ..Default::default()
    };

    assert_eq!(pane.title(), "session:2:my-window");
  }
}

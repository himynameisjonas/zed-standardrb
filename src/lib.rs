use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct StandardRbExtension {}

impl StandardRbExtension {
    fn lsp_path(&mut self, worktree: &Worktree) -> Result<String> {
        let path = worktree.which("standardrb").ok_or_else(|| {
            "standardrb must be installed manually. Install it with `gem install standardrb`."
                .to_string()
        })?;

        Ok(path)
    }
}

impl zed::Extension for StandardRbExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.lsp_path(worktree)?,
            args: vec!["--lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(StandardRbExtension);

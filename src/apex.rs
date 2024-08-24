use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct ApexExtension;

impl zed::Extension for ApexExtension {
    fn new() -> Self {
        Self
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let Some(apex_path) = worktree.which("apex-jorje-lsp.jar") else {
            return Err("Apex not available".to_string());
        };

        Ok(zed::Command {
            command: "java".to_string(),
            args: vec![
                "-cp".to_string(),
                apex_path.to_string(),
                "apex.jorje.lsp.ApexLanguageServerLauncher".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ApexExtension);

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
        // let Some(uber_jar) = worktree.which("apex-jorje-lsp.jar") else {
        //     return Err("Apex JAR not found".into());
        // };
        // let Some(java_path) = worktree.which("java") else {
        //     return Err("java not in path".into());
        // };
        // const APEX_LANGUAGE_SERVER_MAIN: &str = "apex.jorje.lsp.ApexLanguageServerLauncher";
        let Some(uber_jar_binary) = worktree.which("apex-jorje-lsp") else {
            return Err("Binary not in path".into());
        };
        Ok(zed::Command {
            command: uber_jar_binary,
            args: vec![
                // "-jar".to_string(),
                // uber_jar,
                // "-Ddebug.internal.errors=true".to_string(),
                // "-Ddebug.semantic.errors=false".to_string(),
                // APEX_LANGUAGE_SERVER_MAIN.to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ApexExtension);

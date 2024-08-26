use std::path::PathBuf;
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
        //    return Err("Apex JAR not found".into());
        //};
        let uber_jar = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("apex-jorje-lsp.jar");
        let uber_jar_string = uber_jar.to_str().unwrap().to_string();
        let Some(java_path) = worktree.which("java") else {
            return Err("java not in path".into());
        };
        const APEX_LANGUAGE_SERVER_MAIN: &str = "apex.jorje.lsp.ApexLanguageServerLauncher";
        // const LANGUAGE_SERVER_LOG_LEVEL: &str = "ERROR";

        Ok(zed::Command {
            command: java_path.to_string(),
            args: vec![
                "-cp".to_string(),
                uber_jar_string,
                "-Ddebug.internal.errors=true".to_string(),
                "-Ddebug.semantic.errors=false".to_string(),
                APEX_LANGUAGE_SERVER_MAIN.to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ApexExtension);

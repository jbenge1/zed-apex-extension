mod language_servers;

use crate::language_servers::ApexLs;

use zed::lsp::{Completion, Symbol};
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct ApexExtension {
    apex_ls: Option<ApexLs>,
}

impl zed::Extension for ApexExtension {
    fn new() -> Self {
        Self { apex_ls: None }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let Some(uber_jar) = worktree.which("apex-jorje-lsp.jar") else {
            return Err("Apex JAR not found".into());
        };
        let Some(java_path) = worktree.which("java") else {
            return Err("java not in path".into());
        };
        const APEX_LANGUAGE_SERVER_MAIN: &str = "apex.jorje.lsp.ApexLanguageServerLauncher";
        // const LANGUAGE_SERVER_LOG_LEVEL: &str = "ERROR";

        Ok(zed::Command {
            command: java_path.to_string(),
            args: vec![
                "-cp".to_string(),
                uber_jar.to_string(),
                "-Ddebug.internal.errors=true".to_string(),
                "-Ddebug.semantic.errors=${enableSemanticErrors}".to_string(),
                "-Ddebug.completion.statistics=${enableCompletionStatistics}".to_string(),
                "-Dlwc.typegeneration.disabled=true".to_string(),
                APEX_LANGUAGE_SERVER_MAIN.to_string(),
            ],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            ApexLs::LANGUAGE_SERVER_ID => self.apex_ls.as_ref()?.label_for_completion(completion),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            ApexLs::LANGUAGE_SERVER_ID => self.apex_ls.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            ApexLs::LANGUAGE_SERVER_ID => {
                if let Some(apex_ls) = self.apex_ls.as_mut() {
                    return apex_ls.language_server_workspace_configuration(worktree);
                }
            }
            _ => (),
        }

        Ok(None)
    }
}

zed::register_extension!(ApexExtension);

//! Registry of model providers supported by Codex.
//!
//! Providers can be defined in two places:
//!   1. Built-in defaults compiled into the binary so Codex works out-of-the-box.
//!   2. User-defined entries inside `~/.codex/config.toml` under the `model_providers`
//!      key. These override or extend the defaults at runtime.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env::VarError;

use crate::error::EnvVarError;
use crate::openai_api_key::get_openai_api_key;

/// Wire protocol that the provider speaks. Most third-party services only
/// implement the classic OpenAI Chat Completions JSON schema, whereas OpenAI
/// itself (and a handful of others) additionally expose the more modern
/// *Responses* API. The two protocols use different request/response shapes
/// and *cannot* be auto-detected at runtime, therefore each provider entry
/// must declare which one it expects.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WireApi {
    /// The experimental “Responses” API exposed by OpenAI at `/v1/responses`.
    #[default]
    Responses,
    /// Regular Chat Completions compatible with `/v1/chat/completions`.
    Chat,
}

/// Serializable representation of a provider definition.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ModelProviderInfo {
    /// Friendly display name.
    pub name: String,
    /// Base URL for the provider's OpenAI-compatible API.
    pub base_url: String,
    /// Environment variable that stores the user's API key for this provider.
    pub env_key: Option<String>,

    /// Optional instructions to help the user get a valid value for the
    /// variable and set it.
    pub env_key_instructions: Option<String>,

    /// Which wire protocol this provider expects.
    pub wire_api: WireApi,
}

impl ModelProviderInfo {
    /// If `env_key` is Some, returns the API key for this provider if present
    /// (and non-empty) in the environment. If `env_key` is required but
    /// cannot be found, returns an error.
    pub fn api_key(&self) -> crate::error::Result<Option<String>> {
        match &self.env_key {
            Some(env_key) => {
                let env_value = if env_key == crate::openai_api_key::OPENAI_API_KEY_ENV_VAR {
                    get_openai_api_key().map_or_else(|| Err(VarError::NotPresent), Ok)
                } else {
                    std::env::var(env_key)
                };
                env_value
                    .and_then(|v| {
                        if v.trim().is_empty() {
                            Err(VarError::NotPresent)
                        } else {
                            Ok(Some(v))
                        }
                    })
                    .map_err(|_| {
                        crate::error::CodexErr::EnvVar(EnvVarError {
                            var: env_key.clone(),
                            instructions: self.env_key_instructions.clone(),
                        })
                    })
            }
            None => Ok(None),
        }
    }

    /// Returns a bearer token using Azure Entra ID authentication.
    /// Only used when [`api_key()`] returns `Ok(Some("USE_ENTRA_ID"))`.
    pub async fn bearer_token(&self) -> crate::error::Result<String> {
        use azure_core::credentials::TokenCredential;
        use azure_identity::DefaultAzureCredential;

        let credential = DefaultAzureCredential::new().map_err(|e| {
            crate::error::CodexErr::EnvVar(EnvVarError {
                var: "USE_ENTRA_ID".to_string(),
                instructions: Some(e.to_string()),
            })
        })?;
        let token = credential
            .get_token(&["https://cognitiveservices.azure.com/.default"])
            .await
            .map_err(|e| {
                crate::error::CodexErr::EnvVar(EnvVarError {
                    var: "USE_ENTRA_ID".to_string(),
                    instructions: Some(e.to_string()),
                })
            })?;
        Ok(token.token.secret().to_string())
    }
}

/// Built-in default provider list.
pub fn built_in_model_providers() -> HashMap<String, ModelProviderInfo> {
    use ModelProviderInfo as P;

    [
        (
            "openai",
            P {
                name: "OpenAI".into(),
                base_url: "https://api.openai.com/v1".into(),
                env_key: Some("OPENAI_API_KEY".into()),
                env_key_instructions: Some("Create an API key (https://platform.openai.com) and export it as an environment variable.".into()),
                wire_api: WireApi::Responses,
            },
        ),
        (
            "openrouter",
            P {
                name: "OpenRouter".into(),
                base_url: "https://openrouter.ai/api/v1".into(),
                env_key: Some("OPENROUTER_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "azure",
            P {
                name: "Azure OpenAI".into(),
                base_url: "https://YOUR-RESOURCE.openai.azure.com/openai/deployments/YOUR-DEPLOYMENT".into(),
                env_key: Some("AZURE_OPENAI_KEY".into()),
                env_key_instructions: Some("Set to the Azure OpenAI API key or 'USE_ENTRA_ID' for Entra ID auth.".into()),
                wire_api: WireApi::Chat,
            },
        ),
        (
            "gemini",
            P {
                name: "Gemini".into(),
                base_url: "https://generativelanguage.googleapis.com/v1beta/openai".into(),
                env_key: Some("GEMINI_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "ollama",
            P {
                name: "Ollama".into(),
                base_url: "http://localhost:11434/v1".into(),
                env_key: None,
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "mistral",
            P {
                name: "Mistral".into(),
                base_url: "https://api.mistral.ai/v1".into(),
                env_key: Some("MISTRAL_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "deepseek",
            P {
                name: "DeepSeek".into(),
                base_url: "https://api.deepseek.com".into(),
                env_key: Some("DEEPSEEK_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "xai",
            P {
                name: "xAI".into(),
                base_url: "https://api.x.ai/v1".into(),
                env_key: Some("XAI_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
        (
            "groq",
            P {
                name: "Groq".into(),
                base_url: "https://api.groq.com/openai/v1".into(),
                env_key: Some("GROQ_API_KEY".into()),
                env_key_instructions: None,
                wire_api: WireApi::Chat,
            },
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

use crate::provider_catalog;

pub const ABRASAX_API_KEY_ENV: &str = "ABRASAX_API_KEY";
pub const ABRASAX_API_BASE_ENV: &str = "ABRASAX_API_BASE";
pub const ABRASAX_ACCOUNT_ID_ENV: &str = "ABRASAX_ACCOUNT_ID";
pub const ABRASAX_ACCOUNT_EMAIL_ENV: &str = "ABRASAX_ACCOUNT_EMAIL";
pub const ABRASAX_TIER_ENV: &str = "ABRASAX_TIER";
pub const ABRASAX_ENV_FILE: &str = "abrasax-subscription.env";
pub const ABRASAX_CACHE_NAMESPACE: &str = "abrasax-subscription";
pub const ABRASAX_SUBSCRIPTION_ACTIVE_ENV: &str = "ABRASAX_SUBSCRIPTION_ACTIVE";
pub const DEFAULT_ABRASAX_API_BASE: &str = "https://api.abrasax.sh/v1";
pub const ABRASAX_PRICING_URL: &str = "https://abrasax.sh/pricing";
pub const ABRASAX_ACCOUNT_URL: &str = "https://abrasax.sh/account";
pub const ABRASAX_PROVIDER_DISPLAY_NAME: &str = "Abrasax Subscription";
pub const ABRASAX_ROUTE_API_METHOD: &str = "abrasax-subscription";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbrasaxTier {
    Plus,
    Pro,
    Max,
    Ultra,
    Flagship,
}

impl AbrasaxTier {
    pub const ALL: &'static [AbrasaxTier] = &[
        AbrasaxTier::Plus,
        AbrasaxTier::Pro,
        AbrasaxTier::Max,
        AbrasaxTier::Ultra,
        AbrasaxTier::Flagship,
    ];

    pub fn retail_price_usd(self) -> u32 {
        match self {
            Self::Plus => 10,
            Self::Pro => 20,
            Self::Max => 100,
            Self::Ultra => 200,
            Self::Flagship => 1000,
        }
    }

    pub fn usable_budget_usd(self) -> f64 {
        match self {
            Self::Plus => 18.00,
            Self::Pro => 40.00,
            Self::Max => 225.00,
            Self::Ultra => 500.00,
            Self::Flagship => 3000.00,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Plus => "Plus",
            Self::Pro => "Pro",
            Self::Max => "Max",
            Self::Ultra => "Ultra",
            Self::Flagship => "Solo",
        }
    }

    /// Stable machine identifier used for wire values and local persistence.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plus => "plus",
            Self::Pro => "pro",
            Self::Max => "max",
            Self::Ultra => "ultra",
            Self::Flagship => "flagship",
        }
    }

    /// Parse a tier from a wire/persisted value (case-insensitive).
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "plus" => Some(Self::Plus),
            "pro" => Some(Self::Pro),
            "max" => Some(Self::Max),
            "ultra" => Some(Self::Ultra),
            "flagship" | "solo" => Some(Self::Flagship),
            _ => None,
        }
    }

    /// Whether an account on this tier may use a model gated at `required`.
    pub fn allows(self, required: AbrasaxTier) -> bool {
        self >= required
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpstreamRoutingPolicy {
    /// Routing is decided server-side by the abrasax router (model -> provider +
    /// org key). The client does not pick upstreams; this is the only policy for
    /// the managed subscription.
    ServerManaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CuratedModel {
    pub id: &'static str,
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
    pub default_enabled: bool,
    pub routing_policy: UpstreamRoutingPolicy,
    /// Minimum subscription tier that may use this model.
    pub min_tier: AbrasaxTier,
    pub note: &'static str,
}

pub const CURATED_MODELS: &[CuratedModel] = &[
    CuratedModel {
        id: "claude-opus-4-8",
        display_name: "Claude Opus 4.8",
        aliases: &["claude-opus-4-8", "opus-4-8", "opus 4.8", "claude opus 4.8"],
        default_enabled: true,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Frontier model; routed server-side to Anthropic by the abrasax router.",
    },
    CuratedModel {
        id: "claude-opus-5",
        display_name: "Claude Opus 5",
        aliases: &["claude-opus-5", "opus-5", "opus 5", "claude opus 5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Frontier model; routed server-side to Anthropic by the abrasax router.",
    },
    CuratedModel {
        id: "claude-sonnet-4-6",
        display_name: "Claude Sonnet 4.6",
        aliases: &[
            "claude-sonnet-4-6",
            "sonnet-4-6",
            "sonnet 4.6",
            "claude sonnet 4.6",
        ],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Frontier model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "gpt-5.5",
        display_name: "GPT-5.5",
        aliases: &["gpt-5.5", "gpt-5-5", "gpt 5.5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Frontier model; routed server-side to OpenAI by the abrasax router.",
    },
    CuratedModel {
        id: "claude-fable-5",
        display_name: "Claude Fable 5",
        aliases: &["claude-fable-5", "fable-5", "fable 5", "claude fable 5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Ultra,
        note: "Ultra-tier model; routed server-side to Anthropic by the abrasax router.",
    },
    CuratedModel {
        id: "gpt-5.6-sol",
        display_name: "GPT-5.6 Sol",
        aliases: &["gpt-5.6-sol", "gpt 5.6 sol", "sol"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Frontier model; routed server-side to OpenAI by the abrasax router.",
    },
    CuratedModel {
        id: "qwen3-coder-next",
        display_name: "Qwen3 Coder Next",
        aliases: &["qwen3-coder-next", "qwen 3 coder next", "qwen3 coder next"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Open-weight coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "devstral-2-123b",
        display_name: "Devstral 2 123B",
        aliases: &[
            "devstral-2-123b",
            "devstral 2 123b",
            "mistral devstral 2 123b",
        ],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Open-weight coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "deepseek-v3.2",
        display_name: "DeepSeek V3.2",
        aliases: &["deepseek-v3.2", "deepseek v3.2", "deepseek 3.2"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Open-weight reasoning and coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "nova-2-lite",
        display_name: "Amazon Nova 2 Lite",
        aliases: &["nova-2-lite", "nova 2 lite", "amazon nova 2 lite"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Recent efficient multimodal model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "minimax-m2.5",
        display_name: "MiniMax M2.5",
        aliases: &["minimax-m2.5", "minimax m2.5", "minimax m2 5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Recent reasoning and coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "mistral-large-3",
        display_name: "Mistral Large 3",
        aliases: &["mistral-large-3", "mistral large 3"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Open-weight general and coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "kimi-k2.5",
        display_name: "Kimi K2.5",
        aliases: &["kimi-k2.5", "kimi k2.5", "kimi k2 5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Recent reasoning and agentic model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "kimi-k2-thinking",
        display_name: "Kimi K2 Thinking",
        aliases: &["kimi-k2-thinking", "kimi k2 thinking"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Reasoning-focused model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "nemotron-nano-3-30b",
        display_name: "Nemotron Nano 3 30B",
        aliases: &[
            "nemotron-nano-3-30b",
            "nemotron nano 3 30b",
            "nvidia nemotron nano 3 30b",
        ],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Efficient open-weight reasoning model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "gpt-oss-120b",
        display_name: "GPT-OSS 120B",
        aliases: &["gpt-oss-120b", "gpt oss 120b", "openai gpt oss 120b"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Large open-weight reasoning model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "gpt-oss-20b",
        display_name: "GPT-OSS 20B",
        aliases: &["gpt-oss-20b", "gpt oss 20b", "openai gpt oss 20b"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Efficient open-weight reasoning model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "qwen3-next-80b",
        display_name: "Qwen3 Next 80B A3B",
        aliases: &["qwen3-next-80b", "qwen3 next 80b", "qwen 3 next 80b a3b"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Open-weight general and coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "glm-5",
        display_name: "GLM-5",
        aliases: &["glm-5", "glm 5", "zai glm 5"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Recent general and coding model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
    CuratedModel {
        id: "glm-4.7-flash",
        display_name: "GLM 4.7 Flash",
        aliases: &["glm-4.7-flash", "glm 4.7 flash", "zai glm 4.7 flash"],
        default_enabled: false,
        routing_policy: UpstreamRoutingPolicy::ServerManaged,
        min_tier: AbrasaxTier::Plus,
        note: "Efficient recent general model; routed server-side to Amazon Bedrock by the abrasax router.",
    },
];

pub fn curated_models() -> &'static [CuratedModel] {
    CURATED_MODELS
}

pub fn default_model() -> &'static CuratedModel {
    CURATED_MODELS
        .iter()
        .find(|model| model.default_enabled)
        .unwrap_or(&CURATED_MODELS[0])
}

/// Normalize a model id for curated-catalog matching: strips any `@provider`
/// routing suffix, the `[1m]` long-context suffix, and lowercases.
fn normalize_model_key(model: &str) -> String {
    let base = model.trim().split('@').next().unwrap_or("").trim();
    abrasax_provider_core::model_id::canonical(base)
}

pub fn find_curated_model(model: &str) -> Option<&'static CuratedModel> {
    let normalized = normalize_model_key(model);
    CURATED_MODELS.iter().find(|candidate| {
        candidate.id.eq_ignore_ascii_case(&normalized)
            || candidate
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(&normalized))
    })
}

pub fn canonical_model_id(model: &str) -> Option<&'static str> {
    find_curated_model(model).map(|model| model.id)
}

pub fn is_curated_model(model: &str) -> bool {
    canonical_model_id(model).is_some()
}

/// The effective subscription tier for gating decisions.
///
/// `/v1/me` is the source of truth; the last-known tier is persisted to
/// `abrasax-subscription.env` (`ABRASAX_TIER`). Unknown/absent tier behaves like
/// Plus for backward compatibility.
pub fn effective_tier() -> AbrasaxTier {
    cached_tier().unwrap_or(AbrasaxTier::Plus)
}

/// The last tier reported by the backend, if any was persisted.
pub fn cached_tier() -> Option<AbrasaxTier> {
    provider_catalog::load_env_value_from_env_or_config(ABRASAX_TIER_ENV, ABRASAX_ENV_FILE)
        .as_deref()
        .and_then(AbrasaxTier::parse)
}

/// Persist the last-known tier reported by the backend (`None` clears it).
pub fn store_cached_tier(tier: Option<AbrasaxTier>) -> anyhow::Result<()> {
    provider_catalog::save_env_value_to_env_file(
        ABRASAX_TIER_ENV,
        ABRASAX_ENV_FILE,
        tier.map(AbrasaxTier::as_str),
    )
}

/// Whether the current (cached) tier is allowed to use `model`.
/// Non-curated models return `false`.
pub fn is_model_allowed_for_current_tier(model: &str) -> bool {
    find_curated_model(model)
        .map(|curated| effective_tier().allows(curated.min_tier))
        .unwrap_or(false)
}

pub fn routing_policy_detail(model: &CuratedModel) -> String {
    match model.routing_policy {
        UpstreamRoutingPolicy::ServerManaged => {
            "abrasax subscription routing · managed server-side".to_string()
        }
    }
}

pub fn configured_api_key() -> Option<String> {
    provider_catalog::load_env_value_from_env_or_config(ABRASAX_API_KEY_ENV, ABRASAX_ENV_FILE)
}

pub fn configured_api_base() -> Option<String> {
    provider_catalog::load_env_value_from_env_or_config(ABRASAX_API_BASE_ENV, ABRASAX_ENV_FILE)
}

pub fn has_credentials() -> bool {
    configured_api_key().is_some()
}

/// Persist an account API key and its non-secret account metadata in abrasax's
/// owner-only subscription file.
pub fn persist_account_credentials(
    api_key: &str,
    account_id: Option<&str>,
    email: Option<&str>,
    tier: Option<&str>,
) -> anyhow::Result<()> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        anyhow::bail!("refusing to persist an empty abrasax account API key");
    }

    for (key, value) in [
        (ABRASAX_API_KEY_ENV, Some(api_key)),
        (ABRASAX_ACCOUNT_ID_ENV, nonempty(account_id)),
        (ABRASAX_ACCOUNT_EMAIL_ENV, nonempty(email)),
        (ABRASAX_TIER_ENV, nonempty(tier)),
    ] {
        provider_catalog::save_env_value_to_env_file(key, ABRASAX_ENV_FILE, value)?;
    }
    ensure_account_credential_permissions()
}

/// Remove the local account credential and cached account identity/tier. The
/// configured API base is intentionally retained because it is endpoint
/// configuration, not an authorization credential.
pub fn clear_account_credentials() -> anyhow::Result<()> {
    for key in [
        ABRASAX_API_KEY_ENV,
        ABRASAX_ACCOUNT_ID_ENV,
        ABRASAX_ACCOUNT_EMAIL_ENV,
        ABRASAX_TIER_ENV,
    ] {
        provider_catalog::save_env_value_to_env_file(key, ABRASAX_ENV_FILE, None)?;
    }
    clear_runtime_env();
    ensure_account_credential_permissions()
}

fn nonempty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub fn account_credential_path() -> anyhow::Result<std::path::PathBuf> {
    Ok(crate::storage::app_config_dir()?.join(ABRASAX_ENV_FILE))
}

/// Re-harden and verify the subscription file after every credential mutation.
/// This is deliberately an explicit postcondition even though the shared secret
/// writer also applies owner-only permissions.
pub fn ensure_account_credential_permissions() -> anyhow::Result<()> {
    let path = account_credential_path()?;
    if !path.exists() {
        return Ok(());
    }
    crate::storage::harden_secret_file_permissions(&path);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&path)?.permissions().mode() & 0o777;
        if mode & 0o077 != 0 {
            anyhow::bail!(
                "abrasax account credential file has unsafe permissions {:03o}; expected owner-only access",
                mode
            );
        }
    }
    Ok(())
}

pub fn has_router_base() -> bool {
    configured_api_base().is_some()
}

pub fn is_runtime_mode_enabled() -> bool {
    std::env::var(ABRASAX_SUBSCRIPTION_ACTIVE_ENV)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
}

pub fn apply_runtime_env() {
    crate::env::set_var(ABRASAX_SUBSCRIPTION_ACTIVE_ENV, "1");
    crate::env::set_var(
        "ABRASAX_OPENROUTER_API_BASE",
        configured_api_base().unwrap_or_else(|| DEFAULT_ABRASAX_API_BASE.to_string()),
    );
    crate::env::set_var("ABRASAX_OPENROUTER_API_KEY_NAME", ABRASAX_API_KEY_ENV);
    crate::env::set_var("ABRASAX_OPENROUTER_ENV_FILE", ABRASAX_ENV_FILE);
    crate::env::set_var("ABRASAX_OPENROUTER_CACHE_NAMESPACE", ABRASAX_CACHE_NAMESPACE);
    crate::env::set_var("ABRASAX_OPENROUTER_PROVIDER_FEATURES", "0");
    crate::env::set_var("ABRASAX_OPENROUTER_TRANSPORT_STATE", "abrasax-subscription");
    crate::env::remove_var("ABRASAX_OPENROUTER_ALLOW_NO_AUTH");
    crate::env::remove_var("ABRASAX_OPENROUTER_PROVIDER");
    crate::env::remove_var("ABRASAX_OPENROUTER_NO_FALLBACK");
}

pub fn clear_runtime_env() {
    crate::env::remove_var(ABRASAX_SUBSCRIPTION_ACTIVE_ENV);
    crate::env::remove_var("ABRASAX_OPENROUTER_API_BASE");
    crate::env::remove_var("ABRASAX_OPENROUTER_API_KEY_NAME");
    crate::env::remove_var("ABRASAX_OPENROUTER_ENV_FILE");
    crate::env::remove_var("ABRASAX_OPENROUTER_CACHE_NAMESPACE");
    crate::env::remove_var("ABRASAX_OPENROUTER_PROVIDER_FEATURES");
    crate::env::remove_var("ABRASAX_OPENROUTER_TRANSPORT_STATE");
    crate::env::remove_var("ABRASAX_OPENROUTER_ALLOW_NO_AUTH");
    crate::env::remove_var("ABRASAX_OPENROUTER_PROVIDER");
    crate::env::remove_var("ABRASAX_OPENROUTER_NO_FALLBACK");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PLUS_MODELS: &[&str] = &[
        "claude-opus-4-8",
        "claude-opus-5",
        "claude-sonnet-4-6",
        "gpt-5.5",
        "gpt-5.6-sol",
        "qwen3-coder-next",
        "devstral-2-123b",
        "deepseek-v3.2",
        "nova-2-lite",
        "minimax-m2.5",
        "mistral-large-3",
        "kimi-k2.5",
        "kimi-k2-thinking",
        "nemotron-nano-3-30b",
        "gpt-oss-120b",
        "gpt-oss-20b",
        "qwen3-next-80b",
        "glm-5",
        "glm-4.7-flash",
    ];

    #[test]
    fn curated_model_aliases_resolve_to_canonical_ids() {
        assert_eq!(canonical_model_id("opus 4.8"), Some("claude-opus-4-8"));
        assert_eq!(
            canonical_model_id("Claude Opus 4.8"),
            Some("claude-opus-4-8")
        );
        assert_eq!(canonical_model_id("gpt-5.5"), Some("gpt-5.5"));
        assert_eq!(canonical_model_id("GPT 5.5"), Some("gpt-5.5"));
        assert_eq!(
            canonical_model_id("Claude Sonnet 4.6"),
            Some("claude-sonnet-4-6")
        );
        assert_eq!(canonical_model_id("sonnet 4.6"), Some("claude-sonnet-4-6"));
        assert_eq!(canonical_model_id("fable-5"), Some("claude-fable-5"));
        assert_eq!(canonical_model_id("Claude Fable 5"), Some("claude-fable-5"));
        assert_eq!(canonical_model_id("sol"), Some("gpt-5.6-sol"));
        assert_eq!(canonical_model_id("GPT 5.6 Sol"), Some("gpt-5.6-sol"));
        assert_eq!(
            canonical_model_id("Qwen 3 Coder Next"),
            Some("qwen3-coder-next")
        );
        assert_eq!(
            canonical_model_id("Mistral Devstral 2 123B"),
            Some("devstral-2-123b")
        );
        assert_eq!(canonical_model_id("DeepSeek 3.2"), Some("deepseek-v3.2"));
        for (alias, expected) in [
            ("Amazon Nova 2 Lite", "nova-2-lite"),
            ("MiniMax M2.5", "minimax-m2.5"),
            ("Mistral Large 3", "mistral-large-3"),
            ("Kimi K2.5", "kimi-k2.5"),
            ("Kimi K2 Thinking", "kimi-k2-thinking"),
            ("NVIDIA Nemotron Nano 3 30B", "nemotron-nano-3-30b"),
            ("OpenAI GPT OSS 120B", "gpt-oss-120b"),
            ("OpenAI GPT OSS 20B", "gpt-oss-20b"),
            ("Qwen 3 Next 80B A3B", "qwen3-next-80b"),
            ("ZAI GLM 5", "glm-5"),
            ("ZAI GLM 4.7 Flash", "glm-4.7-flash"),
        ] {
            assert_eq!(canonical_model_id(alias), Some(expected), "alias {alias}");
        }
        assert_eq!(canonical_model_id("NVIDIA Nemotron Super 3 120B"), None);
        assert_eq!(canonical_model_id("unknown-model"), None);
    }

    #[test]
    fn curated_model_lookup_ignores_provider_pin_suffix() {
        assert_eq!(
            canonical_model_id("claude-opus-4-8@anthropic"),
            Some("claude-opus-4-8")
        );
        assert_eq!(canonical_model_id("gpt-5.5@openai"), Some("gpt-5.5"));
    }

    #[test]
    fn default_model_is_opus() {
        assert_eq!(default_model().id, "claude-opus-4-8");
    }

    #[test]
    fn curated_catalog_has_exact_paid_and_ultra_sets() {
        assert_eq!(
            CURATED_MODELS
                .iter()
                .filter(|model| model.min_tier == AbrasaxTier::Plus)
                .map(|model| model.id)
                .collect::<Vec<_>>(),
            EXPECTED_PLUS_MODELS
        );
        assert_eq!(
            CURATED_MODELS
                .iter()
                .filter(|model| model.min_tier == AbrasaxTier::Ultra)
                .map(|model| model.id)
                .collect::<Vec<_>>(),
            vec!["claude-fable-5"]
        );
        assert!(
            CURATED_MODELS
                .iter()
                .all(|model| model.min_tier != AbrasaxTier::Flagship)
        );
        assert_eq!(CURATED_MODELS.len(), 20);
        assert!(find_curated_model("magistral-small-1.2").is_none());
        assert!(find_curated_model("gemma-3-27b").is_none());
        assert!(find_curated_model("llama-4-maverick").is_none());
        assert!(find_curated_model("llama-4-scout").is_none());
        assert!(find_curated_model("nemotron-super-3-120b").is_none());
    }

    #[test]
    fn tier_pricing_matches_launched_plans() {
        let expected = [
            (AbrasaxTier::Plus, "plus", "Plus", 10, 18.00),
            (AbrasaxTier::Pro, "pro", "Pro", 20, 40.00),
            (AbrasaxTier::Max, "max", "Max", 100, 225.00),
            (AbrasaxTier::Ultra, "ultra", "Ultra", 200, 500.00),
            (AbrasaxTier::Flagship, "flagship", "Solo", 1000, 3000.00),
        ];

        assert_eq!(AbrasaxTier::ALL, expected.map(|(tier, ..)| tier));
        for (tier, id, display_name, retail_price, usable_budget) in expected {
            assert_eq!(tier.as_str(), id);
            assert_eq!(tier.display_name(), display_name);
            assert_eq!(tier.retail_price_usd(), retail_price);
            assert_eq!(tier.usable_budget_usd(), usable_budget);
        }
    }

    #[test]
    fn tier_parse_round_trips() {
        for tier in AbrasaxTier::ALL {
            assert_eq!(AbrasaxTier::parse(tier.as_str()), Some(*tier));
        }
        assert_eq!(AbrasaxTier::parse("PLUS"), Some(AbrasaxTier::Plus));
        assert_eq!(AbrasaxTier::parse(" Pro "), Some(AbrasaxTier::Pro));
        assert_eq!(AbrasaxTier::parse("MAX"), Some(AbrasaxTier::Max));
        assert_eq!(AbrasaxTier::parse(" ultra "), Some(AbrasaxTier::Ultra));
        assert_eq!(AbrasaxTier::parse(" Flagship "), Some(AbrasaxTier::Flagship));
        assert_eq!(AbrasaxTier::parse(" Solo "), Some(AbrasaxTier::Flagship));
        assert_eq!(AbrasaxTier::parse("starter"), None);
    }

    #[test]
    fn tier_gating_follows_catalog_order() {
        for (account_index, account_tier) in AbrasaxTier::ALL.iter().copied().enumerate() {
            for (required_index, required_tier) in AbrasaxTier::ALL.iter().copied().enumerate() {
                assert_eq!(
                    account_tier.allows(required_tier),
                    account_index >= required_index,
                    "{} gating {}",
                    account_tier.display_name(),
                    required_tier.display_name()
                );
            }
        }
    }

    #[test]
    fn model_entitlements_match_paid_tiers() {
        for model in CURATED_MODELS {
            match model.id {
                "claude-fable-5" => assert_eq!(model.min_tier, AbrasaxTier::Ultra),
                _ => assert_eq!(model.min_tier, AbrasaxTier::Plus),
            }
        }

        for tier in AbrasaxTier::ALL {
            for model in EXPECTED_PLUS_MODELS {
                assert!(tier.allows(find_curated_model(model).unwrap().min_tier));
            }
            assert_eq!(
                tier.allows(find_curated_model("claude-fable-5").unwrap().min_tier),
                matches!(tier, AbrasaxTier::Ultra | AbrasaxTier::Flagship)
            );
        }
    }

    #[test]
    fn effective_tier_defaults_to_plus_when_unknown() {
        let _guard = crate::storage::lock_test_env();
        crate::env::remove_var(ABRASAX_TIER_ENV);
        let temp = tempfile::tempdir().expect("temp home");
        crate::env::set_var("ABRASAX_HOME", temp.path().to_string_lossy().to_string());

        assert_eq!(cached_tier(), None);
        assert_eq!(effective_tier(), AbrasaxTier::Plus);
        for model in EXPECTED_PLUS_MODELS {
            assert!(is_model_allowed_for_current_tier(model));
        }
        assert!(!is_model_allowed_for_current_tier("claude-fable-5"));

        crate::env::set_var(ABRASAX_TIER_ENV, "mystery");
        assert_eq!(cached_tier(), None);
        assert_eq!(effective_tier(), AbrasaxTier::Plus);

        for tier in [AbrasaxTier::Pro, AbrasaxTier::Max] {
            crate::env::set_var(ABRASAX_TIER_ENV, tier.as_str());
            assert_eq!(effective_tier(), tier);
            for model in EXPECTED_PLUS_MODELS {
                assert!(is_model_allowed_for_current_tier(model));
            }
            assert!(!is_model_allowed_for_current_tier("claude-fable-5"));
        }

        crate::env::set_var(ABRASAX_TIER_ENV, AbrasaxTier::Ultra.as_str());
        assert_eq!(effective_tier(), AbrasaxTier::Ultra);
        assert!(is_model_allowed_for_current_tier("claude-fable-5"));

        crate::env::remove_var(ABRASAX_TIER_ENV);
        store_cached_tier(Some(AbrasaxTier::Flagship)).expect("persist tier");
        assert_eq!(cached_tier(), Some(AbrasaxTier::Flagship));
        assert!(is_model_allowed_for_current_tier("claude-fable-5"));
        assert!(is_model_allowed_for_current_tier("gpt-5.6-sol"));

        store_cached_tier(None).expect("clear tier");
        assert_eq!(cached_tier(), None);

        crate::env::remove_var("ABRASAX_HOME");
        crate::env::remove_var(ABRASAX_TIER_ENV);
    }

    #[test]
    fn runtime_mode_flag_tracks_subscription_activation() {
        let _guard = crate::storage::lock_test_env();
        clear_runtime_env();
        assert!(!is_runtime_mode_enabled());

        apply_runtime_env();
        assert!(is_runtime_mode_enabled());

        clear_runtime_env();
        assert!(!is_runtime_mode_enabled());
    }
}

//! Persona / policy / profile merge logic.
//!
//! Defines persona default matrices, effective policy computation,
//! override precedence, and policy constraint checks.
//!
//! This module is expanded in Task 2 of Phase M38.

use crate::entry::types::{HookLevel, Persona, Policy, SourceRewrite, TraceLevel};

// ─── Persona default matrices ──────────────────────────────────────────────────

impl Persona {
    /// Return the default policy for this persona.
    pub fn default_policy(self) -> Policy {
        match self {
            Persona::Runtime => Policy {
                hook_level: HookLevel::None,
                source_rewrite: SourceRewrite::Disabled,
                preload_requirement: crate::entry::types::PreloadRequirement::BestEffort,
                allow_reload: false,
                trace_level: TraceLevel::Off,
                diagnostics_level: crate::entry::types::DiagnosticsLevel::Summary,
                trace_sources: None,
                descriptor_preservation: crate::entry::types::DescriptorPreservation::Strict,
                preserve_native_tostring: true,
                forbid_proxy_on_sensitive_surfaces: true,
                allow_prototype_patch: false,
                allow_function_intrinsic_patch: false,
                cleanup_mode: crate::entry::types::CleanupMode::None,
            },
            Persona::Analysis => Policy {
                hook_level: HookLevel::Transparent,
                source_rewrite: SourceRewrite::Selective,
                preload_requirement: crate::entry::types::PreloadRequirement::BestEffort,
                allow_reload: true,
                trace_level: TraceLevel::Summary,
                diagnostics_level: crate::entry::types::DiagnosticsLevel::Full,
                trace_sources: None,
                descriptor_preservation: crate::entry::types::DescriptorPreservation::BestEffort,
                preserve_native_tostring: true,
                forbid_proxy_on_sensitive_surfaces: false,
                allow_prototype_patch: true,
                allow_function_intrinsic_patch: false,
                cleanup_mode: crate::entry::types::CleanupMode::Reset,
            },
        }
    }

    /// Resolve effective policy: persona defaults overridden by explicit policy fields.
    pub fn merge_policy(self, explicit: Option<Policy>) -> Policy {
        let mut base = self.default_policy();
        if let Some(ex) = explicit {
            if ex.hook_level != HookLevel::None {
                base.hook_level = ex.hook_level;
            }
            if ex.source_rewrite != SourceRewrite::Disabled {
                base.source_rewrite = ex.source_rewrite;
            }
            if ex.trace_level != TraceLevel::Off {
                base.trace_level = ex.trace_level;
            }
            if ex.diagnostics_level != crate::entry::types::DiagnosticsLevel::Off {
                base.diagnostics_level = ex.diagnostics_level;
            }
            base.allow_reload = ex.allow_reload || base.allow_reload;
            if let Some(sources) = ex.trace_sources {
                base.trace_sources = Some(sources);
            }
            if ex.preserve_native_tostring != base.preserve_native_tostring {
                base.preserve_native_tostring = ex.preserve_native_tostring;
            }
            base.forbid_proxy_on_sensitive_surfaces = ex.forbid_proxy_on_sensitive_surfaces;
            base.allow_prototype_patch = ex.allow_prototype_patch || base.allow_prototype_patch;
            base.allow_function_intrinsic_patch =
                ex.allow_function_intrinsic_patch || base.allow_function_intrinsic_patch;
            base.preload_requirement = ex.preload_requirement;
            base.descriptor_preservation = ex.descriptor_preservation;
            base.cleanup_mode = ex.cleanup_mode;
        }
        base
    }

    /// Check whether a given strategy kind is allowed by this persona and its effective policy.
    pub fn allows_strategy(self, policy: &Policy, kind: &super::types::StrategyKind) -> bool {
        match self {
            Persona::Runtime => matches!(
                kind,
                super::types::StrategyKind::WebpackBridge
                    | super::types::StrategyKind::BrowserifyBridge
                    | super::types::StrategyKind::CdpProbe
            ),
            Persona::Analysis => match policy.hook_level {
                HookLevel::Transparent => matches!(
                    kind,
                    super::types::StrategyKind::Dispatch
                        | super::types::StrategyKind::RuntimeTransparent
                        | super::types::StrategyKind::SourceAst
                        | super::types::StrategyKind::SourceRegex
                        | super::types::StrategyKind::WebpackBridge
                        | super::types::StrategyKind::BrowserifyBridge
                        | super::types::StrategyKind::RollupBridge
                        | super::types::StrategyKind::UmdBridge
                        | super::types::StrategyKind::ViteBridge
                        | super::types::StrategyKind::ParcelBridge
                        | super::types::StrategyKind::CdpProbe
                ),
                HookLevel::Aggressive => true,
                HookLevel::None => matches!(
                    kind,
                    super::types::StrategyKind::WebpackBridge
                        | super::types::StrategyKind::BrowserifyBridge
                        | super::types::StrategyKind::ParcelBridge
                        | super::types::StrategyKind::CdpProbe
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::types::StrategyKind;

    #[test]
    fn test_runtime_allows_webpack_and_browserify() {
        let runtime = Persona::Runtime;
        let policy = runtime.default_policy();
        assert!(runtime.allows_strategy(&policy, &StrategyKind::WebpackBridge));
        assert!(runtime.allows_strategy(&policy, &StrategyKind::BrowserifyBridge));
        assert!(runtime.allows_strategy(&policy, &StrategyKind::CdpProbe));
    }

    #[test]
    fn test_runtime_denies_rollup_vite_umd() {
        let runtime = Persona::Runtime;
        let policy = runtime.default_policy();
        assert!(!runtime.allows_strategy(&policy, &StrategyKind::RollupBridge));
        assert!(!runtime.allows_strategy(&policy, &StrategyKind::ViteBridge));
        assert!(!runtime.allows_strategy(&policy, &StrategyKind::UmdBridge));
    }

    #[test]
    fn test_analysis_allows_all_new_strategies() {
        let analysis = Persona::Analysis;
        let policy = analysis.default_policy();
        assert!(analysis.allows_strategy(&policy, &StrategyKind::BrowserifyBridge));
        assert!(analysis.allows_strategy(&policy, &StrategyKind::RollupBridge));
        assert!(analysis.allows_strategy(&policy, &StrategyKind::ViteBridge));
        assert!(analysis.allows_strategy(&policy, &StrategyKind::UmdBridge));
    }

    #[test]
    fn test_analysis_none_hook_allows_webpack_and_browserify() {
        let analysis = Persona::Analysis;
        let mut policy = analysis.default_policy();
        policy.hook_level = HookLevel::None;
        assert!(analysis.allows_strategy(&policy, &StrategyKind::WebpackBridge));
        assert!(analysis.allows_strategy(&policy, &StrategyKind::BrowserifyBridge));
        assert!(!analysis.allows_strategy(&policy, &StrategyKind::RollupBridge));
    }
}

//! Compile-time embedded agent templates.
//!
//! All 30 bundled agent templates are embedded into the binary via `include_str!`.
//! This ensures `rustyhand agent new` works immediately after install — no filesystem
//! discovery needed.

/// Returns all bundled agent templates as `(name, toml_content)` pairs.
pub fn bundled_agents() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "analyst",
            include_str!("../../../agents/analyst/agent.toml"),
        ),
        (
            "architect",
            include_str!("../../../agents/architect/agent.toml"),
        ),
        (
            "assistant",
            include_str!("../../../agents/assistant/agent.toml"),
        ),
        (
            "capability-builder",
            include_str!("../../../agents/capability-builder/agent.toml"),
        ),
        ("coder", include_str!("../../../agents/coder/agent.toml")),
        (
            "coordinator",
            include_str!("../../../agents/coordinator/agent.toml"),
        ),
        (
            "code-reviewer",
            include_str!("../../../agents/code-reviewer/agent.toml"),
        ),
        (
            "customer-support",
            include_str!("../../../agents/customer-support/agent.toml"),
        ),
        (
            "data-scientist",
            include_str!("../../../agents/data-scientist/agent.toml"),
        ),
        (
            "debugger",
            include_str!("../../../agents/debugger/agent.toml"),
        ),
        (
            "devops-lead",
            include_str!("../../../agents/devops-lead/agent.toml"),
        ),
        (
            "diagnostic",
            include_str!("../../../agents/diagnostic/agent.toml"),
        ),
        (
            "doc-writer",
            include_str!("../../../agents/doc-writer/agent.toml"),
        ),
        (
            "email-assistant",
            include_str!("../../../agents/email-assistant/agent.toml"),
        ),
        (
            "health-tracker",
            include_str!("../../../agents/health-tracker/agent.toml"),
        ),
        (
            "hello-world",
            include_str!("../../../agents/hello-world/agent.toml"),
        ),
        (
            "home-automation",
            include_str!("../../../agents/home-automation/agent.toml"),
        ),
        (
            "legal-assistant",
            include_str!("../../../agents/legal-assistant/agent.toml"),
        ),
        (
            "meeting-assistant",
            include_str!("../../../agents/meeting-assistant/agent.toml"),
        ),
        ("ops", include_str!("../../../agents/ops/agent.toml")),
        (
            "orchestrator",
            include_str!("../../../agents/orchestrator/agent.toml"),
        ),
        (
            "personal-finance",
            include_str!("../../../agents/personal-finance/agent.toml"),
        ),
        (
            "planner",
            include_str!("../../../agents/planner/agent.toml"),
        ),
        (
            "recruiter",
            include_str!("../../../agents/recruiter/agent.toml"),
        ),
        (
            "researcher",
            include_str!("../../../agents/researcher/agent.toml"),
        ),
        (
            "sales-assistant",
            include_str!("../../../agents/sales-assistant/agent.toml"),
        ),
        (
            "security-auditor",
            include_str!("../../../agents/security-auditor/agent.toml"),
        ),
        (
            "social-media",
            include_str!("../../../agents/social-media/agent.toml"),
        ),
        (
            "test-engineer",
            include_str!("../../../agents/test-engineer/agent.toml"),
        ),
        (
            "translator",
            include_str!("../../../agents/translator/agent.toml"),
        ),
        (
            "travel-planner",
            include_str!("../../../agents/travel-planner/agent.toml"),
        ),
        ("tutor", include_str!("../../../agents/tutor/agent.toml")),
        ("writer", include_str!("../../../agents/writer/agent.toml")),
    ]
}

/// Install bundled agent templates to `~/.rustyhand/agents/`.
/// Skips any template that already exists on disk (user customization preserved).
pub fn install_bundled_agents(agents_dir: &std::path::Path) {
    for (name, content) in bundled_agents() {
        let dest_dir = agents_dir.join(name);
        let dest_file = dest_dir.join("agent.toml");
        if dest_file.exists() {
            continue; // Preserve user customization
        }
        if std::fs::create_dir_all(&dest_dir).is_ok() {
            let _ = std::fs::write(&dest_file, content);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_hand_types::agent::AgentManifest;

    #[test]
    fn meta_agents_parse_and_use_anthropic() {
        for name in &["coordinator", "capability-builder", "diagnostic"] {
            let toml_src = bundled_agents()
                .into_iter()
                .find(|(n, _)| n == name)
                .unwrap_or_else(|| panic!("{name} template should exist"))
                .1;
            let m: AgentManifest = toml::from_str(toml_src)
                .unwrap_or_else(|e| panic!("{name} template should parse: {e}"));
            assert_eq!(m.name, *name, "{name}: name mismatch");
            assert_eq!(
                m.model.provider, "anthropic",
                "{name}: meta-agents default to Anthropic Sonnet for routing quality"
            );
            assert!(
                m.tags.iter().any(|t| t == "meta"),
                "{name}: should be tagged 'meta' for discovery"
            );
        }
    }

    #[test]
    fn coordinator_can_delegate_to_other_agents() {
        let coord = bundled_agents()
            .into_iter()
            .find(|(n, _)| n == &"coordinator")
            .unwrap()
            .1;
        let m: AgentManifest = toml::from_str(coord).unwrap();
        // Coordinator's whole job is routing — must have agent_send + agent_list.
        assert!(m.capabilities.tools.iter().any(|t| t == "agent_send"));
        assert!(m.capabilities.tools.iter().any(|t| t == "agent_list"));
        // Wildcard message permission so it can talk to any specialist.
        assert!(m.capabilities.agent_message.iter().any(|a| a == "*"));
    }

    #[test]
    fn capability_builder_can_write_skills() {
        let cb = bundled_agents()
            .into_iter()
            .find(|(n, _)| n == &"capability-builder")
            .unwrap()
            .1;
        let m: AgentManifest = toml::from_str(cb).unwrap();
        assert!(m.capabilities.tools.iter().any(|t| t == "file_write"));
        assert!(m.capabilities.tools.iter().any(|t| t == "shell_exec"));
        assert!(m.capabilities.tools.iter().any(|t| t == "web_search"));
    }

    #[test]
    fn diagnostic_reads_audit_but_does_not_modify() {
        let diag = bundled_agents()
            .into_iter()
            .find(|(n, _)| n == &"diagnostic")
            .unwrap()
            .1;
        let m: AgentManifest = toml::from_str(diag).unwrap();
        // Diagnostic uses self_history/self_metrics (real builtin tools)
        // and web_fetch to localhost for the kernel audit API.
        assert!(m.capabilities.tools.iter().any(|t| t == "self_history"));
        assert!(m.capabilities.tools.iter().any(|t| t == "web_fetch"));
        // Read-only: no shell_exec, no agent_spawn.
        assert!(!m.capabilities.tools.iter().any(|t| t == "shell_exec"));
        assert!(!m.capabilities.agent_spawn);
    }

    #[test]
    fn test_assistant_template_has_scheduler_tools() {
        let assistant = bundled_agents()
            .into_iter()
            .find(|(name, _)| *name == "assistant")
            .expect("assistant template should exist")
            .1;

        let manifest: AgentManifest =
            toml::from_str(assistant).expect("assistant template should parse");

        assert!(manifest
            .capabilities
            .tools
            .iter()
            .any(|t| t == "cron_create"));
        assert!(manifest.capabilities.tools.iter().any(|t| t == "cron_list"));
        assert!(manifest
            .capabilities
            .tools
            .iter()
            .any(|t| t == "cron_cancel"));
    }
}

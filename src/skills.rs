use crate::ledger::{hash_bytes, hex};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Bounded, deployable developer "superpowers" for TMIG agents.
///
/// These are capabilities to route work and observations, not claims of
/// unlimited ability. Black-hat items are constrained to authorized adversarial
/// simulation and defensive validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    Dorking,
    CodeWhispering,
    KnowledgeRetrieval,
    VersionControlNavigation,
    BugPrediction,
    RootCauseDebugging,
    ApiOrchestration,
    FrameworkFluency,
    Refactoring,
    TestGeneration,
    LongRunPatience,
    NetworkObservability,
    TechnologyForecasting,
    QuantumClassicalReasoning,
    CodeReview,
    CiCdAutomation,
    SelfHealingInfrastructure,
    SecurityAudit,
    ThreatModeling,
    AuthorizedAdversarialTesting,
    DocumentationGeneration,
    RuntimeMonitoring,
    FeatureFlagging,
    RealtimeDataSync,
    LearningFromLogs,
    LocalCloudEnvironmentProvisioning,
    ComplianceAutomation,
    PrivacyEngineering,
    AccessibilityEngineering,
    WellbeingGuardrails,
    EmbeddedIotEdge,
    ResponsibleAi,
    ContinuousLearning,
    MultiTeamProgramManagement,
    LowNoCodeBridging,
    CrossCulturalLocalization,
    SustainabilityOptimization,
    LegacyModernization,
    MultimodalInterfaceEngineering,
    SimulationAndUat,
    MultiCloudHybridData,
    MarketplaceKnowledgeGraph,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AgentKind {
    Architect,
    Operator,
    Auditor,
    Adversary,
    Swarm,
    Jira,
    Plane,
    FileLayer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillSet {
    pub agent: AgentKind,
    pub skills: BTreeSet<Skill>,
}

impl SkillSet {
    pub fn for_agent(agent: AgentKind) -> Self {
        let mut skills = universal_skills();
        use AgentKind::*;
        use Skill::*;
        match agent {
            Architect => skills.extend([
                FrameworkFluency,
                TechnologyForecasting,
                ResponsibleAi,
                LegacyModernization,
                MultiCloudHybridData,
            ]),
            Operator => skills.extend([
                CiCdAutomation,
                SelfHealingInfrastructure,
                RuntimeMonitoring,
                FeatureFlagging,
                SustainabilityOptimization,
            ]),
            Auditor => skills.extend([
                CodeReview,
                SecurityAudit,
                ComplianceAutomation,
                PrivacyEngineering,
                AccessibilityEngineering,
            ]),
            Adversary => skills.extend([
                ThreatModeling,
                AuthorizedAdversarialTesting,
                BugPrediction,
                NetworkObservability,
                SimulationAndUat,
            ]),
            Swarm => skills.extend([
                RealtimeDataSync,
                LearningFromLogs,
                LongRunPatience,
                CrossCulturalLocalization,
                MultiTeamProgramManagement,
            ]),
            Jira => skills.extend([
                MultiTeamProgramManagement,
                DocumentationGeneration,
                ContinuousLearning,
                MarketplaceKnowledgeGraph,
                WellbeingGuardrails,
            ]),
            Plane => skills.extend([
                ApiOrchestration,
                LocalCloudEnvironmentProvisioning,
                LowNoCodeBridging,
                MultimodalInterfaceEngineering,
                QuantumClassicalReasoning,
            ]),
            FileLayer => skills.extend([
                VersionControlNavigation,
                PrivacyEngineering,
                MultiCloudHybridData,
                EmbeddedIotEdge,
                LegacyModernization,
            ]),
        }
        Self { agent, skills }
    }

    pub fn hash(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("skill set serializes");
        hex(&hash_bytes(&bytes))
    }

    pub fn has(&self, skill: Skill) -> bool {
        self.skills.contains(&skill)
    }
}

pub fn universal_skills() -> BTreeSet<Skill> {
    use Skill::*;
    BTreeSet::from([
        Dorking,
        CodeWhispering,
        KnowledgeRetrieval,
        VersionControlNavigation,
        BugPrediction,
        RootCauseDebugging,
        ApiOrchestration,
        FrameworkFluency,
        Refactoring,
        TestGeneration,
        DocumentationGeneration,
        RuntimeMonitoring,
        SecurityAudit,
        ResponsibleAi,
        ContinuousLearning,
    ])
}

pub fn all_agent_skillsets() -> Vec<SkillSet> {
    use AgentKind::*;
    [
        Architect, Operator, Auditor, Adversary, Swarm, Jira, Plane, FileLayer,
    ]
    .into_iter()
    .map(SkillSet::for_agent)
    .collect()
}

#[cfg(test)]
impl SkillSet {
    /// Universal baseline for agents with no specific role.
    /// Used by the control plane to derive its skills hash.
    pub fn universal_baseline() -> Self {
        Self::for_agent(AgentKind::Architect)
    }
}

mod tests {
    use super::*;

    #[test]
    fn every_agent_has_universal_developer_superpowers() {
        let universal = universal_skills();
        for set in all_agent_skillsets() {
            for skill in &universal {
                assert!(
                    set.skills.contains(skill),
                    "{:?} missing {:?}",
                    set.agent,
                    skill
                );
            }
        }
    }

    #[test]
    fn adversarial_power_is_explicitly_bounded_to_authorized_testing() {
        let set = SkillSet::for_agent(AgentKind::Adversary);
        assert!(set.has(Skill::AuthorizedAdversarialTesting));
        assert!(set.has(Skill::ThreatModeling));
        assert!(set.has(Skill::SecurityAudit));
    }

    #[test]
    fn skillset_hashes_are_stable_and_content_addressed() {
        let a = SkillSet::for_agent(AgentKind::Architect).hash();
        let b = SkillSet::for_agent(AgentKind::Architect).hash();
        let c = SkillSet::for_agent(AgentKind::Operator).hash();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}

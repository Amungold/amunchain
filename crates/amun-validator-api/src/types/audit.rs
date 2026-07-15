use crate::types::id::FindingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditCategory {
    Security,
    Performance,
    Compliance,
    Production,
    Experimental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDomain {
    Installation,
    Security,
    Identity,
    Genesis,
    Storage,
    Network,
    Performance,
    Compliance,
}

impl AuditDomain {
    pub fn all() -> Vec<AuditDomain> {
        vec![
            AuditDomain::Installation,
            AuditDomain::Security,
            AuditDomain::Identity,
            AuditDomain::Genesis,
            AuditDomain::Storage,
            AuditDomain::Network,
            AuditDomain::Performance,
            AuditDomain::Compliance,
        ]
    }
    pub fn category(&self) -> AuditCategory {
        match self {
            AuditDomain::Installation => AuditCategory::Production,
            AuditDomain::Security => AuditCategory::Security,
            AuditDomain::Identity => AuditCategory::Security,
            AuditDomain::Genesis => AuditCategory::Compliance,
            AuditDomain::Storage => AuditCategory::Performance,
            AuditDomain::Network => AuditCategory::Performance,
            AuditDomain::Performance => AuditCategory::Performance,
            AuditDomain::Compliance => AuditCategory::Compliance,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingStatus {
    Pass,
    Fail,
    Warning,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub id: FindingId,
    pub category: AuditCategory,
    pub domain: AuditDomain,
    pub severity: AuditSeverity,
    pub title: String,
    pub description: String,
    pub recommendation: String,
    pub requirement: String,
    pub status: FindingStatus,
    pub evidence: String,
    pub timestamp: u64,
}

pub struct AuditFindingBuilder {
    domain: AuditDomain,
    severity: AuditSeverity,
    title: Option<String>,
    description: Option<String>,
    recommendation: Option<String>,
    requirement: Option<String>,
    status: FindingStatus,
    evidence: Option<String>,
}

impl AuditFindingBuilder {
    pub fn new(domain: AuditDomain, severity: AuditSeverity) -> Self {
        AuditFindingBuilder {
            domain,
            severity,
            title: None,
            description: None,
            recommendation: None,
            requirement: None,
            status: FindingStatus::Pass,
            evidence: None,
        }
    }

    pub fn title(mut self, v: String) -> Self {
        self.title = Some(v);
        self
    }
    pub fn description(mut self, v: String) -> Self {
        self.description = Some(v);
        self
    }
    pub fn recommendation(mut self, v: String) -> Self {
        self.recommendation = Some(v);
        self
    }
    pub fn requirement(mut self, v: String) -> Self {
        self.requirement = Some(v);
        self
    }
    pub fn status(mut self, v: FindingStatus) -> Self {
        self.status = v;
        self
    }
    pub fn evidence(mut self, v: String) -> Self {
        self.evidence = Some(v);
        self
    }

    pub fn build(self) -> AuditFinding {
        AuditFinding {
            id: FindingId::generate(),
            category: self.domain.category(),
            domain: self.domain,
            severity: self.severity,
            title: self.title.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            recommendation: self.recommendation.unwrap_or_default(),
            requirement: self.requirement.unwrap_or_default(),
            status: self.status,
            evidence: self.evidence.unwrap_or_default(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

impl AuditFinding {
    pub fn builder(domain: AuditDomain, severity: AuditSeverity) -> AuditFindingBuilder {
        AuditFindingBuilder::new(domain, severity)
    }
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub validator_id: [u8; 32],
    pub timestamp: u64,
    pub findings: Vec<AuditFinding>,
    pub overall: FindingStatus,
    pub signature: Option<Vec<u8>>,
}

impl AuditReport {
    pub fn passed(&self) -> bool {
        matches!(self.overall, FindingStatus::Pass)
    }
    pub fn critical_findings(&self) -> Vec<&AuditFinding> {
        self.findings
            .iter()
            .filter(|f| {
                matches!(f.severity, AuditSeverity::Critical)
                    && matches!(f.status, FindingStatus::Fail)
            })
            .collect()
    }
    pub fn findings_by_domain(&self, domain: AuditDomain) -> Vec<&AuditFinding> {
        self.findings
            .iter()
            .filter(|f| f.domain == domain)
            .collect()
    }
}

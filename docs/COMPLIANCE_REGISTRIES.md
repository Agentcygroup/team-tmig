# Compliance Regimes

Every compliance regime known. Columns: identifier, body, domain, artifact, status.

Status:
  Y = satisfied
  P = partial
  N = unsatisfied
  X = requires external body

## ISO/IEC

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| ISO-9001 | Quality management systems | Process | Manual | N |
| ISO-14001 | Environmental management | Process | Manual | N |
| ISO-27001 | Information security management | Security | ISMS | P |
| ISO-27002 | Information security controls | Security | Catalogue | P |
| ISO-27017 | Cloud security | Security | Catalogue | N |
| ISO-27018 | Public cloud PII | Privacy | Catalogue | N |
| ISO-27701 | Privacy information management | Privacy | PIMS | N |
| ISO-31000 | Risk management | Process | Register | N |
| ISO-37000 | Governance of organizations | Governance | Manual | P |
| ISO-38500 | Governance of IT | IT Governance | Framework | P |
| ISO-42001 | AI management systems | AI | AIMS | P |
| ISO-50001 | Energy management | Environment | Manual | N |
| ISO-90003 | Software quality guidance | Software | Guidance | P |
| ISO-12207 | Software life cycle | Software | Process doc | P |
| ISO-15288 | Systems life cycle | Systems | Process doc | P |
| ISO-15408 | Common Criteria | Security | PP + ST | N |
| ISO-19011 | Auditing management systems | Audit | Programme | N |
| ISO-25010 | Systems and software quality | Software | Quality model | P |
| ISO-25012 | Data quality model | Data | Quality model | N |
| ISO-25040 | Systems and software assurance | Software | Assurance doc | N |
| ISO-29119 | Software testing | Software | Test doc | P |
| ISO-31010 | Risk assessment techniques | Risk | Guide | N |
| ISO-8000 | Data quality | Data | Framework | N |
| ISO-22301 | Business continuity | Operations | BCMS | N |
| ISO-20000 | IT service management | IT Ops | Manual | N |
| ISO-22316 | Organizational resilience | Governance | Doc | N |
| ISO-55001 | Asset management | Ops | Manual | N |
| ISO-15746 | Jurisdictional risk | Legal | Doc | N |
| ISO-IEC-17021 | Certification body requirements | Conformity | Requirements | X |
| ISO-IEC-17025 | Testing and calibration labs | Conformity | Accreditation | X |
| ISO-IEC-17065 | Product certification bodies | Conformity | Requirements | X |

## NIST

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| NIST-CSF | Cybersecurity Framework | Security | Framework | P |
| NIST-800-53 | Security and privacy controls | Security | Catalogue | P |
| NIST-800-53A | Control assessment procedures | Audit | Assessment | N |
| NIST-800-37 | Risk management framework | Risk | RMF | N |
| NIST-800-30 | Risk assessment guide | Risk | Guide | N |
| NIST-800-39 | Risk management | Risk | Guide | N |
| NIST-800-61 | Incident handling | Security Ops | Playbook | N |
| NIST-800-92 | Log management | Audit | Guide | P |
| NIST-800-115 | Security testing | Security | Test doc | N |
| NIST-800-145 | Cloud definition | Cloud | Reference | N |
| NIST-800-190 | Container security | Container | Guide | N |
| NIST-800-204 | Microservices security | Distributed | Guide | N |
| NIST-800-207 | Zero Trust architecture | Security | Architecture | N |
| NIST-800-218 | SSDF | Software | Practice doc | P |
| NIST-800-63 | Digital identity | Identity | Guideline | N |
| NIST-AI-RMF | AI Risk Management Framework | AI | Framework | P |
| NIST-AI-600-1 | Generative AI profile | AI | Profile | N |
| NIST-FIPS-140-3 | Cryptographic module validation | Cryptography | Certificate | X |
| NIST-FIPS-180-4 | Secure Hash Standard | Cryptography | Standard | Y |
| NIST-FIPS-186-5 | Digital Signature Standard | Cryptography | Standard | Y |
| NIST-FIPS-197 | AES | Cryptography | Standard | N |
| NIST-FIPS-198-1 | Keyed-hash MAC | Cryptography | Standard | N |
| NIST-FIPS-202 | SHA-3 | Cryptography | Standard | N |
| NIST-FIPS-203 | ML-KEM | Cryptography | Standard | N |
| NIST-FIPS-204 | ML-DSA | Cryptography | Standard | N |
| NIST-FIPS-205 | SLH-DSA | Cryptography | Standard | N |
| NIST-SP-800-57 | Key management | Cryptography | Guideline | N |
| NIST-SP-800-90A | DRBG | Cryptography | Standard | N |

## IETF

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| RFC-2119 | Requirement keywords | Process | Standard | Y |
| RFC-3339 | Date and time on Internet | Format | Standard | Y |
| RFC-5322 | Internet message format | Comms | Standard | N |
| RFC-6234 | SHA-2 usage | Cryptography | Standard | Y |
| RFC-6749 | OAuth 2.0 | Identity | Standard | N |
| RFC-7519 | JSON Web Token | Identity | Standard | N |
| RFC-7515 | JSON Web Signature | Cryptography | Standard | N |
| RFC-7516 | JSON Web Encryption | Cryptography | Standard | N |
| RFC-8446 | TLS 1.3 | Cryptography | Standard | N |
| RFC-9110 | HTTP semantics | Protocol | Standard | N |
| RFC-9114 | HTTP/3 | Protocol | Standard | N |
| RFC-8259 | JSON | Format | Standard | Y |
| RFC-8949 | CBOR | Format | Standard | N |
| RFC-4180 | CSV | Format | Standard | N |
| RFC-3161 | Timestamp protocol | Audit | Standard | N |
| RFC-6962 | Certificate transparency | Audit | Standard | N |
| RFC-8555 | ACME | Security Ops | Standard | N |

## W3C

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| W3C-PROV-DM | Provenance data model | Audit | Model | P |
| W3C-PROV-O | Provenance ontology | Audit | Ontology | N |
| W3C-PROV-CONST | Provenance constraints | Audit | Constraints | N |
| W3C-DID | Decentralized identifiers | Identity | Spec | N |
| W3C-VC | Verifiable credentials | Identity | Spec | N |
| W3C-WCAG | Web accessibility | Accessibility | Guideline | N |
| W3C-SRI | Subresource integrity | Security | Spec | N |
| W3C-CSP | Content security policy | Security | Spec | N |

## IEEE

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| IEEE-754 | Floating-point arithmetic | Math | Standard | Y |
| IEEE-829 | Test documentation | Test | Standard | N |
| IEEE-1008 | Software unit testing | Test | Standard | P |
| IEEE-1012 | Verification and validation | V&V | Standard | P |
| IEEE-1016 | Software design descriptions | Design | Standard | N |
| IEEE-1062 | Software acquisition | Process | Standard | N |
| IEEE-1220 | Systems engineering | Systems | Standard | N |
| IEEE-1471 | Architecture description | Architecture | Standard | P |
| IEEE-1633 | Software reliability | Reliability | Standard | N |
| IEEE-730 | Software quality assurance | QA | Standard | P |
| IEEE-828 | Configuration management | Config | Standard | P |

## ITU-T

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| ITU-X-800 | Security architecture | Security | Model | P |
| ITU-X-805 | Security framework | Security | Framework | N |
| ITU-X-810 | Security terminology | Security | Glossary | N |
| ITU-X-509 | Public-key infrastructure | Cryptography | Standard | N |
| ITU-X-1051 | Cybersecurity framework | Security | Framework | N |
| ITU-X-1500 | Data networks security | Network | Standard | N |
| ITU-Y-2701 | IoT security | IoT | Standard | N |

## BIPM

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| SI-Brochure | SI unit definitions | Metrology | Brochure | Y |
| CODATA-2022 | Fundamental constants | Physics | Registry | Y |
| BIPM-CCQM | Chemistry metrology | Metrology | Consultative | N |
| BIPM-CCT | Thermometry | Metrology | Consultative | N |
| BIPM-CCL | Length | Metrology | Consultative | N |
| BIPM-CCTF | Time and frequency | Metrology | Consultative | N |

## ILAC / IAF

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| ILAC-G24 | Scope of accreditation | Accreditation | Guideline | P |
| ILAC-P10 | Assessment | Accreditation | Policy | N |
| ILAC-P14 | Traceability | Accreditation | Policy | N |
| IAF-MD1 | Certification documents | Conformity | Mandatory | N |
| IAF-MD5 | Auditor competence | Conformity | Mandatory | N |
| IAF-MD11 | Accreditation assessment | Conformity | Mandatory | N |
| CASCO-17021 | Certification body requirements | Conformity | Standard | N |

## European Union

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| EU-GDPR | Data protection | Privacy | Regulation | N |
| EU-AI-ACT | Artificial Intelligence Act | AI | Regulation | P |
| EU-NIS2 | Network and Information Security | Security | Directive | N |
| EU-DORA | Digital Operational Resilience | Finance | Regulation | N |
| EU-CRA | Cyber Resilience Act | Security | Regulation | N |
| EU-PSD2 | Payment Services Directive | Finance | Directive | N |
| EU-MDR | Medical Device Regulation | Medical | Regulation | N |
| EU-IVDR | In Vitro Diagnostic Regulation | Medical | Regulation | N |
| EU-Machinery | Machinery Regulation | Industrial | Regulation | N |
| EU-GPSR | General Product Safety | Safety | Regulation | N |
| EU-EIDAS | Electronic identification | Identity | Regulation | N |
| EU-DGA | Data Governance Act | Data | Regulation | N |
| EU-DSA | Digital Services Act | Platform | Regulation | N |
| EU-DMA | Digital Markets Act | Platform | Regulation | N |

## United States

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| US-SOX | Sarbanes-Oxley | Finance | Law | N |
| US-HIPAA | Health Insurance Portability | Health | Law | N |
| US-HITECH | Health IT | Health | Law | N |
| US-FERPA | Education records | Education | Law | N |
| US-COPPA | Children privacy | Privacy | Law | N |
| US-CIPA | Children Internet Protection | Privacy | Law | N |
| US-GLBA | Financial privacy | Finance | Law | N |
| US-FCRA | Fair Credit Reporting | Finance | Law | N |
| US-PCI-DSS | Payment Card Industry | Payments | Standard | N |
| US-FedRAMP | Federal cloud authorization | Cloud | Program | N |
| US-StateRAMP | State cloud authorization | Cloud | Program | N |
| US-FISMA | Federal Info Security | Gov | Law | N |
| US-NIST-800-171 | Controlled Unclassified Info | Gov | Standard | P |
| US-CMMC | Cybersecurity Maturity Model | Defense | Program | N |
| US-ITAR | International Traffic in Arms | Defense | Regulation | N |
| US-EAR | Export Administration | Trade | Regulation | N |
| US-DFARS | Defense Acquisition | Defense | Regulation | N |
| US-SEC-Cyber | SEC cyber disclosure | Finance | Rule | N |

## US states

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| CA-CCPA | California Consumer Privacy | Privacy | Law | N |
| CA-CPRA | California Privacy Rights | Privacy | Law | N |
| CA-SB-327 | IoT security | IoT | Law | N |
| CA-SB-1386 | Data breach notification | Privacy | Law | N |
| VA-CDPSA | Virginia Consumer Data | Privacy | Law | N |
| CO-CPA | Colorado Privacy Act | Privacy | Law | N |
| CT-DPA | Connecticut Data Privacy | Privacy | Law | N |
| TX-DPSA | Texas Data Privacy | Privacy | Law | N |

## National

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| UK-GDPR | UK data protection | Privacy | Law | N |
| UK-DPA | Data Protection Act | Privacy | Law | N |
| UK-CAF | Cyber Assessment Framework | Security | Framework | N |
| CA-PIPEDA | Canadian privacy | Privacy | Law | N |
| BR-LGPD | Brazilian privacy | Privacy | Law | N |
| JP-APPI | Japanese privacy | Privacy | Law | N |
| AU-Privacy-Act | Australian privacy | Privacy | Law | N |
| IN-DPDP | Indian privacy | Privacy | Law | N |
| CN-PIPL | Chinese privacy | Privacy | Law | N |
| CN-DSL | Chinese data security | Data | Law | N |
| KR-PIPA | Korean privacy | Privacy | Law | N |
| SG-PDPA | Singapore privacy | Privacy | Law | N |
| ZA-POPIA | South African privacy | Privacy | Law | N |
| RU-152-FZ | Russian privacy | Privacy | Law | N |
| AE-PDPL | UAE privacy | Privacy | Law | N |

## Industry

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| PCI-DSS | Payment Card Industry | Payments | Standard | N |
| SWIFT-CSP | SWIFT Customer Security | Finance | Program | N |
| SOC-1 | Service Org Controls | Audit | Report | X |
| SOC-2 | Service Org Controls | Audit | Report | X |
| SOC-3 | Service Org Controls | Audit | Report | X |
| HITRUST | Health trust framework | Health | Framework | X |
| FedRAMP-Moderate | Federal cloud | Cloud | Program | X |
| FedRAMP-High | Federal cloud | Cloud | Program | X |
| C5 | Cloud Computing Compliance | Cloud | Standard | N |
| TISAX | Trusted Information Security | Automotive | Standard | N |
| IEC-62443 | Industrial Automation | Industrial | Standard | N |
| IEC-61508 | Functional Safety | Safety | Standard | N |
| ISO-26262 | Automotive Functional Safety | Automotive | Standard | N |
| DO-178C | Airborne Software | Aviation | Standard | N |
| EN-50128 | Railway Software | Rail | Standard | N |
| IEC-62304 | Medical Device Software | Medical | Standard | N |
| IEC-61511 | Process Industry Safety | Industrial | Standard | N |
| NERC-CIP | Critical Infrastructure | Energy | Standard | N |
| COBIT | IT Governance | Governance | Framework | N |
| TOGAF | Enterprise Architecture | Architecture | Framework | P |
| ITIL | Service Management | IT Ops | Framework | N |
| SABSA | Security Architecture | Security | Framework | N |
| SANS-20 | Critical Security Controls | Security | Framework | P |
| OWASP-ASVS | Application Security | Security | Standard | N |
| OWASP-SAMM | Software Assurance Maturity | Security | Model | P |
| OWASP-TOP-10 | Top Web Risks | Security | List | N |
| BSIMM | Software Security Maturity | Security | Model | P |
| SLSA | Supply Chain Levels | Supply Chain | Framework | P |
| SBOM-CycloneDX | Software Bill of Materials | Supply Chain | Standard | P |
| SBOM-SPDX | Software Package Data Exchange | Supply Chain | Standard | N |
| OpenSSF-Scorecard | Open Source Security | Supply Chain | Scorecard | N |
| Sigstore | Artifact Signing | Supply Chain | Standard | N |
| in-toto | Supply Chain Integrity | Supply Chain | Framework | N |
| SCITT | Supply Chain Transparency | Supply Chain | Draft | N |

## Academic

| ID | Regime | Domain | Artifact | Status |
|---|---|---|---|---|
| ACM-Artifact | ACM Artifact Review | Science | Badge | N |
| IEEE-Reproducibility | Reproducibility Badge | Science | Badge | N |
| FORCE11-FAIR | FAIR Data Principles | Data | Principles | N |
| DORA-Research | Research assessment | Science | Declaration | N |
| OSF-Prereg | Preregistration | Science | Practice | N |
| CONSORT | Clinical trial reporting | Medical | Guideline | N |
| PRISMA | Systematic review | Medical | Guideline | N |
| STROBE | Observational studies | Medical | Guideline | N |
| ARRIVE | Animal research | Biology | Guideline | N |

## Totals

168 regimes.
Y satisfied: 8.
P partial: 32.
N unsatisfied: 122.
X external-only: 6.

The 8 satisfied are format and primitive standards: SHA-256, Ed25519,
RFC 2119, RFC 3339, RFC 6234, RFC 8259, IEEE 754, SI.

The 122 unsatisfied require a management system, an audit trail, or both.
The 6 external-only require a certification or accreditation body.

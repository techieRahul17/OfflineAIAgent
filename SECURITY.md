# Security Policy

Thank you for your interest in the security of OfflineAIAgent. This document explains how to report vulnerabilities, the supported versions, and our response timelines.

If you discover a security issue
- DO NOT open a public issue. Instead, report it privately.
- Preferred: Create a confidential security advisory via GitHub (if you have access) or contact the repository owner directly.
- If you must use email, send details to the repository owner at: security@techieRahul17.invalid (replace with a real contact if available). Public PRs or issues may expose sensitive details and delay remediation.

What to include in your report
- A clear description of the vulnerability and its impact.
- Steps to reproduce with code or proof-of-concept (preferably minimal).
- Tested versions and environment details (OS, Python version, model/runtime versions).
- Any suggested mitigations or fixes (if available).
- Contact details so we may follow up.

Response process
- Acknowledgement: You will receive an initial acknowledgement within 72 hours.
- Triage & fix: We will triage and work on a fix based on severity and available resources. Critical issues will be prioritized.
- Disclosure: We will coordinate disclosure and give time for users to upgrade before public disclosure.

Supported versions
- We support the current main branch and the latest release. Please specify the commit or tag you tested against.

Security best practices for users
- Run the project on trusted, patched systems.
- Store model binaries and data on secured storage with least-privilege access.
- Avoid committing secrets to the repository. Use environment variables or secure vaults.
- Validate and sanitize inputs if you expose agent functionality to untrusted users.

Third-party models and licenses
- OfflineAIAgent is a framework. Models and weights are typically third-party artifacts with their own licenses and restrictions. Always comply with the model license and distribution terms.

Reporting escalation
If you do not receive a response within 7 days for a high-severity issue, you may escalate by posting a secure advisory via GitHub (if available) or contacting the maintainer’s public channels.

Thank you for helping keep OfflineAIAgent secure.

Designed and maintained by Rahul V S — GitHub: [techieRahul17](https://github.com/techieRahul17)

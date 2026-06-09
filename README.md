### ImageGuard Scan
GitHub Action to scan container images and generate ImageVulnerabilityReport
This action integrates into your CI pipeline to scan container images using Trivy and generate a Kubernetes ImageVulnerabilityReport Custom Resource for ImageGuard.
Features

### Features

Scans container images with Trivy (official action)
- Generates structured ImageVulnerabilityReport YAML
- Creates a Pull Request with the report for review and ArgoCD reconciliation
- Supports both image tags and digests
- Designed for GitOps workflows

### Usage 
```
- name: ImageGuard Scan
   uses: ShravanKamble/imageguard-scan@v1
   with:
     image: 'myapp:latest'
     report-path: '.imageguard/reports/'
```

| Parameter | Description | Required | Default |
| -------- | -------- | -------- | -------- |
| `image` | Container image to scan (e.g. myapp:latest)  | yes | - |
| `path` | Folder where the ImageVulnerability(Custom Resource) Report would be created | no | /imageguard/reports |

### How It Works

- Scans the provided container image using Trivy
- Parses the scan report and generates a Kubernetes ImageVulnerabilityReport Custom Resource
- Commits the report and creates a Pull Request for manual review and merging

### Technologies Used

- Trivy (official GitHub Action)
- Python (report parser)

import json

with open("./result.json") as f:
    report = json.load(f)

    summary = {"CRITICAL": 0, "HIGH": 0, "MEDIUM": 0, "LOW": 0}

    for result in report.get('Results', []):
        for vuln in result.get('Vulnerabilities', []):
            sev = vuln.get('Severity')
            if sev in summary:
                summary[sev] += 1

    with open("./resource.yml", "w") as f:
        f.write(f"apiVersion: imageguard.k8s.io/v1\nkind: ImageVulnerability\nmetadata:\n  name: nginx-latest-vulnerability\nspec:\n  image: nginx:latest\n  summary:\n    critical: {summary.get('CRITICAL')}\n    high: {summary.get('HIGH')}\n    medium: {summary.get('MEDIUM')}\n    low: {summary.get('LOW')}\n")
        f.close()
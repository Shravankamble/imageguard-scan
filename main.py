import json

with open("./result.json") as f:
    report = json.load(f)

    summary = {"CRITICAL": 0, "HIGH": 0, "MEDIUM": 0, "LOW": 0}

    for result in report.get('Results', []):
        for vuln in result.get('Vulnerabilities', []):
            sev = vuln.get('Severity')
            if sev in summary:
                summary[sev] += 1

    if summary.get("HIGH") >= 10:
        print("lots of vulnerability")
    else:
        print("ok to be deployed in production")
---
name: skill-example-generator
description: Procedure for enhancing a skill with example scripts for each mode and updating documentation
source: auto-skill
extracted_at: '2026-05-31T21:48:55.080Z'
---

# Adding Example Scripts to a Skill

When you have a skill that defines modes, commands, or procedures, it is helpful to provide ready‑to‑run example scripts so users can immediately see how to apply the skill. This skill outlines a reusable approach for:

1. Creating one example script per mode/procedure.
2. Placing the scripts in the skill’s directory.
3. Making the scripts executable.
4. Updating the skill’s `SKILL.md` to document the new examples.

## Step‑by‑step

### 1. Identify the modes/procedures
List the distinct modes, commands, or protocols that the skill describes (e.g., Scenario Forecast, Non‑Intrusive Observation, Diagnostic Report, Six‑Helper).

### 2. Create a script for each
For each mode, write a small script (bash, Python, etc.) that:
- Demonstrates the core idea using the tools or APIs referenced by the skill.
- Is self‑contained and safe to run in a demo environment (use dummy data or placeholders where needed).
- Includes a shebang and brief comments explaining what it shows.

Place the script in the skill’s folder, naming it after the mode in snake_case with a `.sh` (or appropriate) extension.

### 3. Make scripts executable
After creating each script, run:
```bash
chmod +x <script_name>.sh
```
so users can execute it directly.

### 4. Update the skill documentation
Edit the skill’s `SKILL.md` file to add a section (e.g., “ตัวอย่างสคริปต์ (Example Scripts)”) that:
- Briefly describes each script.
- Lists the script file names and what they demonstrate.
- Mentions that the scripts are starting points and can be adapted.

### 5. Verify
- Run each script to ensure they work without errors.
- Confirm that the updated `SKILL.md` renders correctly and the links (if any) are accurate.

## Reusability
This procedure can be applied to any skill that defines discrete operational modes. By following these steps, you turn abstract guidance into hands‑on examples, lowering the barrier to adoption.

## Example (from the observability-doctor skill)
- `scenario_forecast.sh` – shows how to fetch telemetry and compute a simple forecast.
- `non_intrusive_observation.sh` – demonstrates passive observation using an Agent Tracker‑like approach.
- `diagnostic_report.sh` – generates a JSONL diagnostic report and a placeholder heatmap.
- `six_helper.sh` – walks through a Six Hats Thinking session on a user‑provided topic.

Each script was made executable and referenced in the updated `SKILL.md` under a new “Example Scripts” section.

---
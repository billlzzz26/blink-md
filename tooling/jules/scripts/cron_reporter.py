import subprocess
from datetime import datetime

# cron_reporter.py: Generate Markdown report for Hermes Cron Jobs

REPORT_PATH = "docs/reports/CRON_DASHBOARD.md"

def get_cron_data():
    try:
        result = subprocess.run(["hermes", "cron", "list"], capture_output=True, text=True)
        return result.stdout
    except Exception as e:
        return f"Error fetching cron data: {e}"

def generate_report():
    data = get_cron_data()
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    report = f"""# 🌙 Hermes Cron Dashboard
<!-- PRIORITY:1 -->
Generated at: {timestamp}

## 📋 Active Jobs List
```text
{data}
```

## 🛠 Next Actions
- [ ] ตรวจสอบ Logs สำหรับ Job ที่มีปัญหา
- [ ] ปรับจูนเวลาการรันตามความเหมาะสม

---
*รายงานนี้ถูกสร้างขึ้นโดยระบบอัตโนมัติเพื่อความโปร่งใสในการทำงานของ Agent*
"""
    
    with open(REPORT_PATH, "w") as f:
        f.write(report)
    print(f"✅ Cron Dashboard generated at {REPORT_PATH}")

if __name__ == "__main__":
    generate_report()

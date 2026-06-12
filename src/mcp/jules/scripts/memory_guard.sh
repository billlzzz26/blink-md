#!/usr/bin/env bash
# memory_guard.sh: Automated Context Monitoring
# Version: 1.0.0
# Logic: Monitor -> Alert -> Propose (Never auto-delete)

THRESHOLD_KB=200
CONTEXT_FILES=("GEMINI.md" "conductor/" "docs/")
REPORT_FILE="docs/MEMORY_REPORT.md"

echo "🔍 Running Memory Guard Check..."

TOTAL_SIZE=0
for item in "${CONTEXT_FILES[@]}"; do
    if [ -e "$item" ]; then
        SIZE=$(du -sk "$item" | cut -f1)
        TOTAL_SIZE=$((TOTAL_SIZE + SIZE))
    fi
done

echo "📊 Total Context Size: ${TOTAL_SIZE}KB (Threshold: ${THRESHOLD_KB}KB)"

if [ "$TOTAL_SIZE" -gt "$THRESHOLD_KB" ]; then
    echo "⚠️  Threshold exceeded! Generating Pruning Proposal..."
    cat <<EOF > "$REPORT_FILE"
# 🚨 Pruning Proposal: Memory Threshold Exceeded
<!-- PRIORITY:1 -->
Generated at: $(date)

## Status
- Current Size: ${TOTAL_SIZE}KB
- Threshold: ${THRESHOLD_KB}KB

## Recommendations
- [ ] ตรวจสอบประวัติงานใน \`conductor/tracks/\` และย้ายอันที่เสร็จแล้วไป Archived
- [ ] สรุปเนื้อหาใน \`docs/\` ที่เป็นงานวิจัยเก่า
- [ ] รันกระบวนการ Staging Review ตาม \`docs/STAGING_REVIEW_PROTOCOL.md\`

**Action Required:** อาจารย์กรุณาตรวจสอบและอนุมัติการสรุปความจำครับ
EOF
    echo "✅ Proposal generated at $REPORT_FILE"
else
    echo "✅ Memory usage is healthy."
fi

#!/usr/bin/env bash
# สคริปต์สำหรับทดสอบ Constriction Protocol (10 Points)
set -e

HOOK_PATH=".gemini/hooks/boost_check.sh"
PASS_COUNT=0
FAIL_COUNT=0

test_case() {
    local name="$1"
    local prompt="$2"
    local expected_type="$3"

    echo -n "🧪 Testing [$name]: '$prompt' -> "
    RESULT=$(printf '{"prompt": "%s"}' "$prompt" | bash "$HOOK_PATH")
    
    if [[ "$expected_type" == "ALLOW" ]]; then
        if [[ "$RESULT" == *"\"decision\": \"allow\""* ]]; then
            echo "✅ PASS (Allowed)"
            PASS_COUNT=$((PASS_COUNT + 1))
        else
            echo "❌ FAIL (Should have allowed)"
            echo "   Output: $RESULT"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    else
        if [[ "$RESULT" == *"PIPELINE TRIGGERED"* ]]; then
            echo "✅ PASS (Triggered Boost)"
            if [[ "$RESULT" == *"💡 สกิลที่แนะนำ"* ]]; then echo "   ✨ Found Skill Recommendation!"; fi
            PASS_COUNT=$((PASS_COUNT + 1))
        else
            echo "❌ FAIL (Should have triggered Boost)"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    fi
}

echo "--- Starting Constriction Protocol Validation ---"

# 1. Top-Level Constraints (Rule #4)
test_case "Absolute Rule" "ใช้ภาษาไทยเท่านั้น" "ALLOW"
test_case "Strict Prohibition" "ห้ามใช้ไลบรารีนอก" "ALLOW"

# 2. Output formatting (Rule #2)
test_case "Markdown Spec" "สรุปเป็น # หัวข้อ markdown" "ALLOW"

# 3. Natural Syntax (Rule #1)
test_case "Numbered List" "1. ทำ A\n2. ทำ B" "ALLOW"

# 4. Skill Discovery Hinting
test_case "Skill Hint: Prompt" "ช่วยเขียน prompt ดีๆ" "BOOST"
test_case "Skill Hint: Files" "แมพไฟล์ในโปรเจกต์" "BOOST"

# 5. Low Detail (Should Boost)
test_case "Very Vague" "ทำห่าอะไร" "BOOST"

echo "---------------------------------------"
echo "📊 Summary: $PASS_COUNT Passed, $FAIL_COUNT Failed"

[ $FAIL_COUNT -eq 0 ]

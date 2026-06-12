import unittest
import sys
import os
import importlib.util

# Dynamic import for hidden .gemini/hooks directory
spec = importlib.util.spec_from_file_location("intent_optimizer", ".gemini/hooks/intent_optimizer.py")
intent_optimizer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(intent_optimizer)
analyze_intent = intent_optimizer.analyze_intent

class TestIntentOptimizer(unittest.TestCase):
    def test_basic_intent(self):
        result = analyze_intent("ขอแผนการทำโปรเจกต์")
        self.assertEqual(result['score'], 23) # บรีฟ(0)+แผน(10)+ทำ(5)+โปรเจกต์(8) = 23
        self.assertIn("แผน", result['topics'])

    def test_empty_input(self):
        result = analyze_intent("")
        self.assertEqual(result['score'], 0)
        self.assertEqual(len(result['topics']), 0)

    def test_no_keywords(self):
        result = analyze_intent("วันนี้อากาศดีนะ")
        self.assertEqual(result['score'], 0)

    def test_duplicate_keywords(self):
        # ควรนับแค่อันเดียว
        result = analyze_intent("แผน แผน แผน")
        self.assertEqual(result['score'], 10)

    def test_special_characters(self):
        result = analyze_intent("hook!! หรือ intent??")
        self.assertIn("hook", result['topics'])
        self.assertIn("intent", result['topics'])
        self.assertEqual(result['score'], 19)

if __name__ == '__main__':
    unittest.main()

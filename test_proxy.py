#!/usr/bin/env python3
"""
Gemini Proxy Service Test Script
Tests the functionality of the Gemini API proxy service
"""

import requests
import json
import sys
from datetime import datetime
from typing import Dict, Any, Optional

class GeminiProxyTester:
    def __init__(self, base_url: str = "http://127.0.0.1:5675"):
        self.base_url = base_url
        self.session = requests.Session()
        self.test_results = []
        
    def log_test(self, test_name: str, success: bool, details: str = ""):
        """Log test results"""
        result = {
            "test": test_name,
            "success": success,
            "timestamp": datetime.now().isoformat(),
            "details": details
        }
        self.test_results.append(result)
        status = "✓" if success else "✗"
        print(f"{status} {test_name}: {details}")
    
    def test_health_check(self) -> bool:
        """Test health check endpoint"""
        try:
            response = self.session.get(f"{self.base_url}/health", timeout=5)
            
            if response.status_code == 200:
                data = response.json()
                if data.get("status") == "healthy":
                    self.log_test("Health Check", True, f"Service is healthy - {data.get('service', 'Unknown')}")
                    return True
                else:
                    self.log_test("Health Check", False, f"Unexpected status: {data.get('status')}")
                    return False
            else:
                self.log_test("Health Check", False, f"HTTP {response.status_code}")
                return False
                
        except requests.exceptions.RequestException as e:
            self.log_test("Health Check", False, f"Connection error: {str(e)}")
            return False
    
    def test_list_models(self, api_key: Optional[str] = None) -> bool:
        """Test list models endpoint"""
        try:
            headers = {}
            if api_key:
                headers["Authorization"] = f"Bearer {api_key}"
            
            response = self.session.get(f"{self.base_url}/v1beta/models", headers=headers, timeout=10)
            
            if response.status_code == 200:
                data = response.json()
                if "models" in data:
                    model_count = len(data["models"])
                    self.log_test("List Models", True, f"Found {model_count} models")
                    return True
                else:
                    self.log_test("List Models", False, "No models field in response")
                    return False
            elif response.status_code == 401:
                self.log_test("List Models", False, "Authentication required")
                return False
            else:
                self.log_test("List Models", False, f"HTTP {response.status_code}")
                return False
                
        except requests.exceptions.RequestException as e:
            self.log_test("List Models", False, f"Connection error: {str(e)}")
            return False
    
    def test_get_model(self, model_name: str = "gemini-2.5-pro", api_key: Optional[str] = None) -> bool:
        """Test get specific model endpoint"""
        try:
            headers = {}
            if api_key:
                headers["Authorization"] = f"Bearer {api_key}"
            
            response = self.session.get(f"{self.base_url}/v1beta/models/{model_name}", headers=headers, timeout=10)
            
            if response.status_code == 200:
                data = response.json()
                if "name" in data:
                    self.log_test("Get Model", True, f"Model info retrieved: {data.get('name')}")
                    return True
                else:
                    self.log_test("Get Model", False, "No name field in response")
                    return False
            elif response.status_code == 401:
                self.log_test("Get Model", False, "Authentication required")
                return False
            else:
                self.log_test("Get Model", False, f"HTTP {response.status_code}")
                return False
                
        except requests.exceptions.RequestException as e:
            self.log_test("Get Model", False, f"Connection error: {str(e)}")
            return False
    
    def test_generate_content(self, model_name: str = "gemini-2.5-pro", api_key: Optional[str] = None) -> bool:
        """Test content generation endpoint"""
        try:
            headers = {"Content-Type": "application/json"}
            if api_key:
                headers["Authorization"] = f"Bearer {api_key}"
            
            payload = {
                "contents": [
                    {
                        "parts": [
                            {
                                "text": "Hello, this is a test message. Please respond with 'Test successful'."
                            }
                        ]
                    }
                ]
            }
            
            response = self.session.post(
                f"{self.base_url}/v1beta/models/{model_name}:generateContent",
                headers=headers,
                json=payload,
                timeout=30
            )
            
            if response.status_code == 200:
                data = response.json()
                if "candidates" in data:
                    self.log_test("Generate Content", True, "Content generated successfully")
                    return True
                else:
                    self.log_test("Generate Content", False, "No candidates in response")
                    return False
            elif response.status_code == 401:
                self.log_test("Generate Content", False, "Authentication required")
                return False
            else:
                self.log_test("Generate Content", False, f"HTTP {response.status_code}: {response.text}")
                return False
                
        except requests.exceptions.RequestException as e:
            self.log_test("Generate Content", False, f"Connection error: {str(e)}")
            return False
    
    def test_stream_generate_content(self, model_name: str = "gemini-2.5-pro", api_key: Optional[str] = None) -> bool:
        """Test streaming content generation endpoint"""
        try:
            headers = {"Content-Type": "application/json"}
            if api_key:
                headers["Authorization"] = f"Bearer {api_key}"
            
            payload = {
                "contents": [
                    {
                        "parts": [
                            {
                                "text": "Count from 1 to 5, one number per line."
                            }
                        ]
                    }
                ]
            }
            
            response = self.session.post(
                f"{self.base_url}/v1beta/models/{model_name}:streamGenerateContent",
                headers=headers,
                json=payload,
                timeout=30,
                stream=True
            )
            
            if response.status_code == 200:
                chunks_received = 0
                for line in response.iter_lines():
                    if line:
                        chunks_received += 1
                        if chunks_received >= 3:  # Stop after a few chunks
                            break
                
                if chunks_received > 0:
                    self.log_test("Stream Generate Content", True, f"Received {chunks_received} chunks")
                    return True
                else:
                    self.log_test("Stream Generate Content", False, "No chunks received")
                    return False
            elif response.status_code == 401:
                self.log_test("Stream Generate Content", False, "Authentication required")
                return False
            else:
                self.log_test("Stream Generate Content", False, f"HTTP {response.status_code}")
                return False
                
        except requests.exceptions.RequestException as e:
            self.log_test("Stream Generate Content", False, f"Connection error: {str(e)}")
            return False
    
    def run_all_tests(self, api_key: Optional[str] = None) -> Dict[str, Any]:
        """Run all tests and return results"""
        print(f"🚀 Starting Gemini Proxy Tests at {self.base_url}")
        print("=" * 50)
        
        # Test 1: Health Check (no auth required)
        health_ok = self.test_health_check()
        
        if not health_ok:
            print("\n❌ Health check failed. Service may not be running.")
            return self.get_summary()
        
        # Test 2: List Models
        self.test_list_models(api_key)
        
        # Test 3: Get Model
        self.test_get_model(api_key=api_key)
        
        # Test 4: Generate Content
        self.test_generate_content(api_key=api_key)
        
        # Test 5: Stream Generate Content
        self.test_stream_generate_content(api_key=api_key)
        
        return self.get_summary()
    
    def get_summary(self) -> Dict[str, Any]:
        """Get test summary"""
        total = len(self.test_results)
        passed = sum(1 for r in self.test_results if r["success"])
        failed = total - passed
        
        summary = {
            "total_tests": total,
            "passed": passed,
            "failed": failed,
            "success_rate": f"{(passed/total*100):.1f}%" if total > 0 else "0%",
            "results": self.test_results
        }
        
        print("\n" + "=" * 50)
        print(f"📊 Test Summary: {passed}/{total} tests passed ({summary['success_rate']})")
        
        if failed > 0:
            print(f"❌ {failed} tests failed")
            for result in self.test_results:
                if not result["success"]:
                    print(f"   - {result['test']}: {result['details']}")
        else:
            print("✅ All tests passed!")
        
        return summary

def main():
    """Main function"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Test Gemini Proxy Service")
    parser.add_argument("--url", default="http://127.0.0.1:5675", help="Base URL of the proxy service")
    parser.add_argument("--api-key",default="123456", help="API key for authentication")
    parser.add_argument("--output", help="Output file for test results (JSON)")
    
    args = parser.parse_args()
    
    tester = GeminiProxyTester(args.url)
    results = tester.run_all_tests(args.api_key)
    
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"\n📁 Results saved to {args.output}")
    
    # Exit with non-zero code if tests failed
    sys.exit(0 if results["failed"] == 0 else 1)

if __name__ == "__main__":
    main()
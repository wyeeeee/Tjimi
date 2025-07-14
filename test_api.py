#!/usr/bin/env python3
"""
Test script for the Gemini API proxy
"""
import requests
import json
import time

BASE_URL = "http://127.0.0.1:5675"

def test_health():
    """Test health check endpoint"""
    try:
        response = requests.get(f"{BASE_URL}/health")
        print(f"Health check: {response.status_code}")
        if response.status_code == 200:
            print(f"Response: {response.json()}")
        return response.status_code == 200
    except Exception as e:
        print(f"Health check failed: {e}")
        return False

def test_models():
    """Test models endpoint"""
    try:
        response = requests.get(f"{BASE_URL}/v1/models")
        print(f"Models: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            if 'models' in data:
                print(f"Found {len(data['models'])} models")
                for model in data['models'][:3]:  # Show first 3 models
                    print(f"  - {model.get('name', 'Unknown')}")
            else:
                print(f"Response: {data}")
        else:
            print(f"Error: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"Models test failed: {e}")
        return False

def test_generate_content():
    """Test content generation endpoint"""
    try:
        # Use the correct Gemini API format according to documentation
        payload = {
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": "Hello, how are you? Please respond in one sentence."
                        }
                    ]
                }
            ]
        }
        
        response = requests.post(
            f"{BASE_URL}/v1/models/gemini-1.5-flash/generateContent",
            json=payload,
            headers={"Content-Type": "application/json"}
        )
        
        print(f"Generate content: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            if 'candidates' in data and len(data['candidates']) > 0:
                content = data['candidates'][0].get('content', {})
                parts = content.get('parts', [])
                if parts and 'text' in parts[0]:
                    print(f"Generated response: {parts[0]['text'][:100]}...")
                    print("✅ Content generation successful")
                else:
                    print(f"Unexpected response format: {data}")
            else:
                print(f"No candidates in response: {data}")
        else:
            print(f"❌ Error {response.status_code}: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"❌ Generate content test failed: {e}")
        return False

def test_streaming_content():
    """Test streaming content generation endpoint"""
    try:
        payload = {
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": "Count from 1 to 5 with explanations."
                        }
                    ]
                }
            ]
        }
        
        response = requests.post(
            f"{BASE_URL}/v1/models/gemini-1.5-flash/streamGenerateContent",
            json=payload,
            headers={"Content-Type": "application/json"},
            stream=True
        )
        
        print(f"Streaming content: {response.status_code}")
        if response.status_code == 200:
            print("📡 Streaming response:")
            chunks_received = 0
            for line in response.iter_lines():
                if line:
                    line_str = line.decode('utf-8')
                    if line_str.startswith('data: '):
                        chunks_received += 1
                        if chunks_received <= 3:  # Show first 3 chunks
                            print(f"  Chunk {chunks_received}: {line_str[:80]}...")
                        if chunks_received >= 10:  # Limit to avoid spam
                            break
            
            print(f"✅ Received {chunks_received} streaming chunks")
            return chunks_received > 0
        else:
            print(f"❌ Error {response.status_code}: {response.text}")
            return False
    except Exception as e:
        print(f"❌ Streaming test failed: {e}")
        return False

def main():
    """Run all tests"""
    print("Testing Gemini API Proxy...")
    print("=" * 50)
    
    # Wait a bit for server to start
    time.sleep(2)
    
    tests = [
        ("Health Check", test_health),
        ("Models List", test_models),
        ("Generate Content", test_generate_content),
        ("Streaming Content", test_streaming_content),
    ]
    
    results = []
    for name, test_func in tests:
        print(f"\n🧪 Testing {name}...")
        result = test_func()
        results.append((name, result))
        print(f"{'✅ PASSED' if result else '❌ FAILED'}")
    
    print("\n" + "=" * 50)
    print("📊 Test Results:")
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    for name, result in results:
        status = "✅ PASSED" if result else "❌ FAILED"
        print(f"  {name}: {status}")
    
    print(f"\n🎯 Overall: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! The proxy is working correctly.")
    else:
        print("⚠️  Some tests failed. Check the logs for details.")

if __name__ == "__main__":
    main()
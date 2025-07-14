#!/usr/bin/env python3
"""
Gemini API Proxy 测试脚本
测试代理服务器的各种功能
"""

import requests
import json
import time
import sys

# 代理服务器地址
PROXY_BASE_URL = "http://127.0.0.1:5675"

def test_health_check():
    """测试健康检查端点"""
    print("🔍 测试健康检查...")
    try:
        response = requests.get(f"{PROXY_BASE_URL}/health", timeout=10)
        if response.status_code == 200:
            print("✅ 健康检查通过")
            print(f"   响应: {response.json()}")
        else:
            print(f"❌ 健康检查失败: {response.status_code}")
            print(f"   响应: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"❌ 健康检查异常: {e}")
        return False

def test_list_models():
    """测试获取模型列表"""
    print("\n📋 测试获取模型列表...")
    try:
        response = requests.get(f"{PROXY_BASE_URL}/v1/models", timeout=15)
        if response.status_code == 200:
            models = response.json()
            print("✅ 获取模型列表成功")
            if 'models' in models:
                print(f"   可用模型数量: {len(models['models'])}")
                for model in models['models'][:3]:  # 显示前3个模型
                    print(f"   - {model.get('name', 'Unknown')}")
                if len(models['models']) > 3:
                    print(f"   ... 还有 {len(models['models']) - 3} 个模型")
            else:
                print(f"   响应: {models}")
        else:
            print(f"❌ 获取模型列表失败: {response.status_code}")
            print(f"   响应: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"❌ 获取模型列表异常: {e}")
        return False

def test_get_model_info():
    """测试获取特定模型信息"""
    print("\n🔍 测试获取模型信息...")
    model_name = "gemini-1.5-flash"
    try:
        response = requests.get(f"{PROXY_BASE_URL}/v1/models/{model_name}", timeout=15)
        if response.status_code == 200:
            model_info = response.json()
            print(f"✅ 获取模型 {model_name} 信息成功")
            print(f"   模型名称: {model_info.get('name', 'Unknown')}")
            print(f"   显示名称: {model_info.get('displayName', 'Unknown')}")
            print(f"   版本: {model_info.get('version', 'Unknown')}")
        else:
            print(f"❌ 获取模型信息失败: {response.status_code}")
            print(f"   响应: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"❌ 获取模型信息异常: {e}")
        return False

def test_generate_content():
    """测试内容生成 (非流式)"""
    print("\n💬 测试内容生成 (非流式)...")
    
    payload = {
        "contents": [
            {
                "parts": [
                    {
                        "text": "你好！请用一句话介绍什么是人工智能。"
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 100
        }
    }
    
    try:
        start_time = time.time()
        response = requests.post(
            f"{PROXY_BASE_URL}/v1/models/gemini-1.5-flash/generateContent",
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=30
        )
        end_time = time.time()
        
        if response.status_code == 200:
            result = response.json()
            print("✅ 内容生成成功")
            print(f"   响应时间: {end_time - start_time:.2f}s")
            
            if 'candidates' in result and result['candidates']:
                content = result['candidates'][0].get('content', {})
                if 'parts' in content and content['parts']:
                    text = content['parts'][0].get('text', '')
                    print(f"   生成内容: {text[:100]}{'...' if len(text) > 100 else ''}")
                else:
                    print(f"   响应结构: {result}")
            else:
                print(f"   完整响应: {result}")
        else:
            print(f"❌ 内容生成失败: {response.status_code}")
            print(f"   响应: {response.text}")
        return response.status_code == 200
    except Exception as e:
        print(f"❌ 内容生成异常: {e}")
        return False

def test_streaming_content():
    """测试流式内容生成"""
    print("\n🌊 测试流式内容生成...")
    
    payload = {
        "contents": [
            {
                "parts": [
                    {
                        "text": "请写一个关于春天的简短诗歌，不超过4行。"
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.8,
            "maxOutputTokens": 200
        }
    }
    
    try:
        start_time = time.time()
        headers = {
            "Content-Type": "application/json",
            "Accept": "text/event-stream",
            "Cache-Control": "no-cache"
        }
        
        print("   发送流式请求...")
        response = requests.post(
            f"{PROXY_BASE_URL}/v1/models/gemini-1.5-flash/streamGenerateContent",
            json=payload,
            headers=headers,
            stream=True,
            timeout=60
        )
        
        print(f"   响应状态码: {response.status_code}")
        print(f"   响应头: {dict(response.headers)}")
        
        if response.status_code == 200:
            print("✅ 流式生成开始")
            chunks_received = 0
            total_content = ""
            raw_data_received = ""
            
            try:
                for line in response.iter_lines(decode_unicode=False):
                    if line:
                        line_text = line.decode('utf-8', errors='ignore')
                        print(f"   原始行: {repr(line_text)}")
                        raw_data_received += line_text + "\n"
                        
                        if line_text.startswith('data: '):
                            data_content = line_text[6:]  # 移除 'data: ' 前缀
                            chunks_received += 1
                            print(f"   数据块 {chunks_received}: {data_content[:100]}...")
                            
                            if data_content.strip() and data_content.strip() != "[DONE]":
                                try:
                                    data = json.loads(data_content)
                                    
                                    # 提取文本内容
                                    if 'candidates' in data and data['candidates']:
                                        candidate = data['candidates'][0]
                                        if 'content' in candidate and 'parts' in candidate['content']:
                                            for part in candidate['content']['parts']:
                                                if 'text' in part:
                                                    total_content += part['text']
                                                    print(f"   新文本: {part['text']}")
                                except json.JSONDecodeError as je:
                                    print(f"   JSON解析错误: {je}")
                                    print(f"   原始数据: {data_content}")
                        elif line_text.strip():
                            print(f"   非数据行: {line_text}")
                            
            except Exception as stream_e:
                print(f"   流处理异常: {stream_e}")
                return False
            
            end_time = time.time()
            print(f"   接收到 {chunks_received} 个数据块")
            print(f"   响应时间: {end_time - start_time:.2f}s")
            if total_content.strip():
                print(f"   生成内容: {total_content.strip()}")
            else:
                print("   未接收到文本内容")
                print(f"   原始数据: {raw_data_received[:500]}...")
            
            return chunks_received > 0  # 如果收到数据块就认为成功
        else:
            print(f"❌ 流式生成失败: {response.status_code}")
            print(f"   响应: {response.text}")
            return False
    except requests.exceptions.Timeout:
        print("❌ 流式生成超时")
        return False
    except Exception as e:
        print(f"❌ 流式生成异常: {e}")
        import traceback
        print(f"   详细错误: {traceback.format_exc()}")
        return False

def test_error_handling():
    """测试错误处理"""
    print("\n⚠️  测试错误处理...")
    
    # 测试无效的模型名称
    try:
        response = requests.post(
            f"{PROXY_BASE_URL}/v1/models/invalid-model/generateContent",
            json={"contents": [{"parts": [{"text": "test"}]}]},
            headers={"Content-Type": "application/json"},
            timeout=15
        )
        
        if response.status_code == 404:
            print("✅ 无效模型错误处理正确")
        else:
            print(f"⚠️  无效模型返回状态码: {response.status_code}")
        
        return True
    except Exception as e:
        print(f"❌ 错误处理测试异常: {e}")
        return False

def main():
    """主函数"""
    print("🚀 Gemini API Proxy 测试开始")
    print("=" * 50)
    
    tests = [
        ("健康检查", test_health_check),
        ("模型列表", test_list_models),
        ("模型信息", test_get_model_info),
        ("内容生成", test_generate_content),
        ("流式生成", test_streaming_content),
        ("错误处理", test_error_handling),
    ]
    
    results = []
    
    for test_name, test_func in tests:
        try:
            result = test_func()
            results.append((test_name, result))
        except KeyboardInterrupt:
            print("\n\n⏹️  测试被用户中断")
            break
        except Exception as e:
            print(f"\n❌ 测试 {test_name} 发生未知错误: {e}")
            results.append((test_name, False))
    
    # 打印测试结果汇总
    print("\n" + "=" * 50)
    print("📊 测试结果汇总:")
    
    passed = 0
    total = len(results)
    
    for test_name, result in results:
        status = "✅ 通过" if result else "❌ 失败"
        print(f"   {test_name}: {status}")
        if result:
            passed += 1
    
    print(f"\n🎯 总计: {passed}/{total} 个测试通过")
    
    if passed == total:
        print("🎉 所有测试通过！代理服务器运行正常。")
        return 0
    else:
        print("⚠️  部分测试失败，请检查代理服务器和API密钥配置。")
        return 1

if __name__ == "__main__":
    try:
        exit_code = main()
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print("\n\n👋 测试脚本退出")
        sys.exit(0)
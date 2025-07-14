#!/usr/bin/env python3
"""
检查数据库中的数据
"""

import sqlite3
import os
import tempfile
import re

# 数据库路径
db_path = os.path.join(tempfile.gettempdir(), "gemini_proxy.db")

def parse_timestamp(timestamp_str):
    """简单的时间戳解析函数"""
    try:
        # 移除纳秒部分，只保留微秒
        # 从 2025-07-14T14:11:35.396231100+00:00 到 2025-07-14T14:11:35.396231+00:00
        cleaned = re.sub(r'(\.\d{6})\d+', r'\1', timestamp_str)
        # 移除时区信息进行简单解析
        dt_part = cleaned.split('+')[0].split('Z')[0]
        return dt_part.replace('T', ' ')
    except:
        return timestamp_str

def check_database():
    print(f"🔍 检查数据库: {db_path}")
    
    if not os.path.exists(db_path):
        print("❌ 数据库文件不存在")
        return
    
    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()
        
        print("\n📊 数据库表结构:")
        cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
        tables = cursor.fetchall()
        for table in tables:
            print(f"   - {table[0]}")
        
        print("\n🔑 API 密钥:")
        cursor.execute("SELECT id, name, key_value, is_active, usage_count FROM api_keys")
        api_keys = cursor.fetchall()
        if api_keys:
            for key in api_keys:
                key_display = f"{key[2][:8]}...{key[2][-8:]}" if len(key[2]) > 16 else key[2]
                status = "活跃" if key[3] else "禁用"
                print(f"   - {key[1]}: {key_display} ({status}, 使用次数: {key[4]})")
        else:
            print("   没有 API 密钥")
        
        print("\n📝 请求日志:")
        cursor.execute("""
            SELECT 
                rl.id,
                ak.name as api_key_name,
                rl.method,
                rl.path,
                rl.status_code,
                rl.response_time_ms,
                rl.created_at
            FROM request_logs rl
            JOIN api_keys ak ON rl.api_key_id = ak.id
            ORDER BY rl.created_at DESC
            LIMIT 10
        """)
        logs = cursor.fetchall()
        if logs:
            print(f"   总共找到 {len(logs)} 条日志记录:")
            for log in logs:
                time_str = parse_timestamp(log[6])
                print(f"   - {log[2]} {log[3]} -> {log[4]} ({log[5]}ms) [{log[1]}] {time_str}")
        else:
            print("   没有请求日志")
        
        # 检查日志总数
        cursor.execute("SELECT COUNT(*) FROM request_logs")
        total_logs = cursor.fetchone()[0]
        print(f"\n📈 统计信息:")
        print(f"   总日志数: {total_logs}")
        
        print("\n🔧 应用设置:")
        cursor.execute("SELECT id, created_at FROM app_settings")
        settings = cursor.fetchall()
        if settings:
            for setting in settings:
                time_str = parse_timestamp(setting[1])
                print(f"   - 设置ID: {setting[0]}, 创建时间: {time_str}")
        else:
            print("   没有应用设置")
            
        conn.close()
        
    except Exception as e:
        print(f"❌ 数据库检查失败: {e}")

if __name__ == "__main__":
    check_database()
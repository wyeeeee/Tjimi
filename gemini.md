Gemini API 核心使用文档
1. 总览
本文件旨在提供 Gemini API 的核心功能说明，主要着重于如何通过 API 密钥 (API Key) 进行文本生成。这是一个 RESTful API，使用 JSON 格式进行数据交换，并支持两种主要的交互模式：非流式 (Unary) 和流式 (Streaming)。

2. 身份验证 (Authentication)
所有对 Gemini API 的请求都必须经过身份验证。验证方式是通过在 HTTP 请求中附加 API 密钥。

方式: 将 API 密钥作为 URL 的一个查询参数 (query parameter)。

参数名称: key

示例 URL: https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=YOUR_API_KEY

请务必将 YOUR_API_KEY 替换为您自己的有效密钥。

3. 请求模式
Gemini API 支持两种请求模式，您可以根据应用场景选择最合适的一种。

非流式 (Unary): 发送一次完整的请求，等待模型处理完毕后，一次性接收完整的响应。适用于不需要实时反馈的场景。

流式 (Streaming): 发送一次请求后，服务器会以数据流的形式，陆续返回一个个内容“块”(chunks)。这可以让用户更快地看到响应的初始部分，提供更实时的交互体验。

4. 非流式 (Unary) 请求
这是最简单的交互模式。

4.1. 端点 (Endpoint)
HTTP 方法: POST

URL: https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent

模型 {model}: 推荐使用 gemini-1.5-flash 或 gemini-pro。

4.2. 请求格式 (Request Body)
请求的主体 (body) 必须是 Content-Type: application/json 的 JSON 对象。核心结构包含一个 contents 数组。

{
  "contents": [
    {
      "role": "user",
      "parts": [
        {
          "text": "请用简体中文写一首关于宇宙的短诗。"
        }
      ]
    }
  ]
}

(关于 contents 结构的详细说明，请参考多轮对话的示例)

4.3. 响应格式 (Response Body)
API 会一次性返回包含完整生成内容的 JSON 对象。您需要解析 candidates[0].content.parts[0].text 来获取最终文本。

{
  "candidates": [
    {
      "content": {
        "role": "model",
        "parts": [
          {
            "text": "银河旋臂轻舒展，\n星尘微光舞翩跹。\n黑洞深邃藏谜语，\n时空涟漪过客船。"
          }
        ]
      },
      "finishReason": "STOP",
      "index": 0,
      "safetyRatings": [ /* ... */ ]
    }
  ]
}

4.4. cURL 示例
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=YOUR_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
          "contents": [{
            "role": "user",
            "parts":[{
              "text": "请解释什么是 API？"
            }]
          }]
        }'

5. 流式 (Streaming) 请求
此模式通过服务器发送事件 (Server-Sent Events, SSE) 实现。

5.1. 端点 (Endpoint)
HTTP 方法: POST

URL: https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent?alt=sse

关键参数: alt=sse 是启用流式响应的必需参数。

模型 {model}: 推荐使用 gemini-1.5-flash 或 gemini-pro。

5.2. 请求格式 (Request Body)
请求的 JSON 主体与非流式请求 完全相同。

5.3. 响应格式 (Response Format)
服务器会返回一个数据流，由多个数据块组成。每个块都以 data:  开头，后面跟着一个 JSON 对象。您需要监听这个数据流，解析每一个 JSON 块，并将所有块中的 text 内容拼接起来，才能获得完整的响应。

流式响应示例:

data: {"candidates":[{"content":{"role":"model","parts":[{"text":"银河旋"}]},"finishReason":"NULL","index":0,"safetyRatings":[/*...*/]}]}

data: {"candidates":[{"content":{"role":"model","parts":[{"text":"臂轻舒展，\n"}]},"finishReason":"NULL","index":0,"safetyRatings":[/*...*/]}]}

data: {"candidates":[{"content":{"role":"model","parts":[{"text":"星尘微光舞翩跹。\n"}]},"finishReason":"NULL","index":0,"safetyRatings":[/*...*/]}]}

data: {"candidates":[{"content":{"role":"model","parts":[{"text":"黑洞深邃藏谜语，\n"}]},"finishReason":"NULL","index":0,"safetyRatings":[/*...*/]}]}

data: {"candidates":[{"content":{"role":"model","parts":[{"text":"时空涟漪过客船。"}]},"finishReason":"STOP","index":0,"safetyRatings":[/*...*/]}]}


处理逻辑:

读取响应流。

对于每一行，检查是否以 data:  开头。

如果是，则提取该行 data:  后面的 JSON 字符串。

解析该 JSON，并提取 candidates[0].content.parts[0].text 的值。

将提取到的文本片段追加到您的最终结果中。

当 finishReason 变为 STOP 时，表示流已结束。

5.4. cURL 示例
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:streamGenerateContent?key=YOUR_API_KEY&alt=sse" \
    -H "Content-Type: application/json" \
    -d '{
          "contents": [{
            "role": "user",
            "parts":[{
              "text": "用一个很长的段落解释什么是人工智能。"
            }]
          }]
        }'

6. 错误处理
如果请求失败，API 会返回一个包含 error 对象的 JSON，这在流式和非流式模式下都适用。

code: HTTP 状态码 (例如 400, 429, 500)。

message: 字符串。错误的详细描述。

status: 字符串。错误的状态码名称 (例如 INVALID_ARGUMENT)。

错误响应示例 (JSON)
{
  "error": {
    "code": 400,
    "message": "API key not valid. Please pass a valid API key.",
    "status": "INVALID_ARGUMENT"
  }
}

常见错误:

400 Bad Request: 请求格式错误 (例如，无效的 JSON) 或 API 密钥无效。

429 Resource Exhausted: 请求频率过高，已超出您的配额。您的轮询项目需要特别注意这个错误，并在发生时采取退避策略 (backoff strategy)。
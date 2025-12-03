# 验证码识别系统 API 文档

## 📋 目录

- [概述](#概述)
- [基础信息](#基础信息)
- [ddddocr 引擎 API](#ddddocr-引擎-api)
- [AntiCAP 引擎 API](#anticap-引擎-api)
- [系统 API](#系统-api)
- [错误处理](#错误处理)
- [示例代码](#示例代码)

---

## 概述

本系统集成了 **ddddocr** 和 **AntiCAP** 两大验证码识别引擎，提供多种验证码识别服务。

### 支持的验证码类型

| 类型 | ddddocr | AntiCAP |
|------|---------|---------|
| 普通验证码 | ✅ | ✅ |
| 算术验证码 | ⚠️ | ✅ |
| 目标检测 | ✅ | ✅ |
| 滑块验证码 | ✅ | ✅ |
| 点选验证码 | ✅ | ❌ |
| 旋转验证码 | ❌ | ✅ |
| 图片相似度 | ❌ | ✅ |

---

## 基础信息

### 服务地址
```
http://localhost:8000
```

### 请求格式
- Content-Type: `application/json`
- 图片格式: Base64 编码（支持带或不带 `data:image/png;base64,` 前缀）

### 响应格式
所有 API 返回 JSON 格式，包含 `success` 字段表示是否成功。

---

## ddddocr 引擎 API

### 1. 普通验证码识别

**接口地址**: `POST /api/ocr`

**功能**: 识别常规验证码文字，支持标准、旧版、Beta 三种模式

**请求参数**:
```json
{
  "image": "data:image/png;base64,iVBORw0KG...",
  "type": "normal"  // normal | old | beta
}
```

**参数说明**:
- `image` (string, 必填): Base64 编码的图片
- `type` (string, 可选): 识别模式
  - `normal`: 标准模式（默认）
  - `old`: 旧版模式，兼容老式验证码
  - `beta`: Beta 模式，高精度识别

**响应示例**:
```json
{
  "success": true,
  "result": "AB3D",
  "type": "normal",
  "mode": "标准模式",
  "length": 4
}
```

---

### 2. 目标检测

**接口地址**: `POST /api/det`

**功能**: 检测图片中的所有目标并识别每个目标的内容

**请求参数**:
```json
{
  "image": "data:image/png;base64,iVBORw0KG..."
}
```

**响应示例**:
```json
{
  "success": true,
  "objects": [
    {
      "id": 1,
      "bbox": [10, 20, 50, 60],
      "center": [30, 40],
      "label": "猫",
      "width": 40,
      "height": 40
    }
  ],
  "count": 1,
  "description": "检测到 1 个目标"
}
```

---

### 3. 滑块验证码

**接口地址**: `POST /api/slide`

**功能**: 计算滑块需要移动的距离

**请求参数**:
```json
{
  "target": "data:image/png;base64,iVBORw0KG...",
  "background": "data:image/png;base64,iVBORw0KG..."
}
```

**参数说明**:
- `target`: 滑块图片（缺口部分）
- `background`: 背景图片

**响应示例**:
```json
{
  "success": true,
  "distance": 120,
  "distance_px": "120px",
  "background_size": {"width": 300, "height": 150},
  "target_size": {"width": 60, "height": 60},
  "description": "滑块需要向右移动 120 像素"
}
```

---

### 4. 点选验证码

**接口地址**: `POST /api/click`

**功能**: 识别需要点击的目标位置和类别

**请求参数**:
```json
{
  "image": "data:image/png;base64,iVBORw0KG...",
  "question": "请点击所有的猫"  // 可选
}
```

**响应示例**:
```json
{
  "success": true,
  "question": "请点击所有的猫",
  "targets": [
    {
      "id": 1,
      "position": {"x": 100, "y": 150},
      "bbox": [80, 130, 120, 170],
      "label": "猫",
      "type": "文字",
      "size": {"width": 40, "height": 40}
    }
  ],
  "count": 1,
  "description": "识别到 1 个可点击目标",
  "image_size": {"width": 300, "height": 200}
}
```

---

## AntiCAP 引擎 API

### 1. 通用 OCR 识别

**接口地址**: `POST /api/anticap/ocr`

**功能**: 通用验证码识别、算术验证码、图标/文字检测、旋转验证码

**请求参数**:
```json
{
  "image": "data:image/png;base64,iVBORw0KG...",
  "type": "ocr"  // ocr | math | detection_icon | detection_text | single_rotate
}
```

**参数说明**:
- `type` (string, 必填): 识别类型
  - `ocr`: 通用 OCR 识别
  - `math`: 算术验证码（返回计算结果）
  - `detection_icon`: 图标检测（返回坐标列表）
  - `detection_text`: 文字检测（返回坐标列表）
  - `single_rotate`: 单图旋转验证码（返回旋转角度）

**响应示例 (OCR)**:
```json
{
  "success": true,
  "result": "5X9K",
  "type": "ocr",
  "description": "通用 OCR 识别",
  "engine": "AntiCAP"
}
```

**响应示例 (算术)**:
```json
{
  "success": true,
  "result": "7",
  "type": "math",
  "description": "算术验证码识别（返回计算结果）",
  "engine": "AntiCAP"
}
```

**响应示例 (检测)**:
```json
{
  "success": true,
  "result": [
    [{"class": "Text", "box": [10.5, 20.3, 50.2, 60.8]}],
    [{"class": "Text", "box": [60.1, 20.5, 100.3, 61.2]}]
  ],
  "type": "detection_text",
  "description": "文字检测（返回坐标列表）",
  "engine": "AntiCAP"
}
```

**响应示例 (旋转)**:
```json
{
  "success": true,
  "result": 45,
  "type": "single_rotate",
  "description": "单图旋转验证码（返回旋转角度）",
  "engine": "AntiCAP"
}
```

---

### 2. 滑块验证码

**接口地址**: `POST /api/anticap/slide`

**功能**: OpenCV 算法识别缺口滑块或阴影滑块

**请求参数**:
```json
{
  "target": "data:image/png;base64,iVBORw0KG...",
  "background": "data:image/png;base64,iVBORw0KG...",
  "mode": "match"  // match | comparison
}
```

**参数说明**:
- `mode` (string, 必填): 滑块类型
  - `match`: 缺口滑块
  - `comparison`: 阴影滑块

**响应示例**:
```json
{
  "success": true,
  "result": {"target": [115, 50]},
  "mode": "match",
  "description": "缺口滑块识别",
  "engine": "AntiCAP (OpenCV)"
}
```

---

### 3. 双图旋转验证码

**接口地址**: `POST /api/anticap/rotate`

**功能**: 识别内外圆环需要旋转的角度

**请求参数**:
```json
{
  "inside": "data:image/png;base64,iVBORw0KG...",
  "outside": "data:image/png;base64,iVBORw0KG..."
}
```

**参数说明**:
- `inside`: 内圆图片
- `outside`: 外圆图片

**响应示例**:
```json
{
  "success": true,
  "similarity": 0.665,
  "inner_angle": 75.5,
  "raw_angle": 151,
  "engine": "AntiCAP (OpenCV)"
}
```

**字段说明**:
- `similarity`: 相似度（0-1之间）
- `inner_angle`: 内圆需要旋转的角度
- `raw_angle`: 原始角度

---

### 4. 图片相似度对比

**接口地址**: `POST /api/anticap/compare`

**功能**: 使用孪生神经网络计算两张图片的相似度

**请求参数**:
```json
{
  "image1": "data:image/png;base64,iVBORw0KG...",
  "image2": "data:image/png;base64,iVBORw0KG..."
}
```

**响应示例**:
```json
{
  "success": true,
  "similarity": 0.8523,
  "description": "图片相似度（0-1之间，越接近1越相似）",
  "engine": "AntiCAP (孪生神经网络)"
}
```

---

## 系统 API

### 1. 健康检查

**接口地址**: `GET /api/health`

**功能**: 检查服务运行状态和可用引擎

**响应示例**:
```json
{
  "status": "ok",
  "message": "服务运行正常",
  "engines": {
    "ddddocr": "✅ 已加载",
    "anticap": "✅ 已加载"
  },
  "features": {
    "ddddocr": {
      "ocr": "普通验证码识别（标准/旧版/Beta）",
      "detection": "目标检测与识别",
      "slide": "滑块验证码",
      "click": "点选验证码"
    },
    "anticap": {
      "ocr": "通用 OCR 识别",
      "math": "算术验证码",
      "detection": "图标/文字检测",
      "slide": "滑块验证码（缺口/阴影）",
      "rotate": "旋转验证码",
      "compare": "图片相似度对比"
    }
  }
}
```

---

### 2. 查看可用模型

**接口地址**: `GET /api/models`

**功能**: 列出所有可用的识别模型

**响应示例**:
```json
{
  "success": true,
  "models": {
    "ddddocr": {
      "normal": {
        "name": "标准模式",
        "description": "适用于常规验证码"
      },
      "old": {
        "name": "旧版模式",
        "description": "兼容老式验证码"
      },
      "beta": {
        "name": "Beta 模式",
        "description": "高精度识别"
      }
    },
    "anticap": {
      "ocr": {
        "name": "通用 OCR",
        "description": "识别各类验证码文字"
      },
      "math": {
        "name": "算术验证码",
        "description": "识别并计算算术表达式"
      }
      // ... 更多模型
    }
  }
}
```

---

## 错误处理

### 错误响应格式

```json
{
  "detail": "错误描述信息"
}
```

### 常见错误码

| 状态码 | 说明 |
|--------|------|
| 400 | 请求参数错误 |
| 500 | 服务器内部错误 |
| 503 | 服务不可用（引擎未加载） |

---

## 示例代码

### Python 示例

```python
import requests
import base64

# 读取图片并转换为 base64
with open("captcha.png", "rb") as f:
    image_base64 = base64.b64encode(f.read()).decode('utf-8')

# 1. ddddocr 普通验证码识别
response = requests.post(
    "http://localhost:8000/api/ocr",
    json={
        "image": f"data:image/png;base64,{image_base64}",
        "type": "normal"
    }
)
result = response.json()
print(f"识别结果: {result['result']}")

# 2. AntiCAP 算术验证码
response = requests.post(
    "http://localhost:8000/api/anticap/ocr",
    json={
        "image": f"data:image/png;base64,{image_base64}",
        "type": "math"
    }
)
result = response.json()
print(f"计算结果: {result['result']}")

# 3. 滑块验证码
with open("target.png", "rb") as f:
    target_base64 = base64.b64encode(f.read()).decode('utf-8')
with open("background.png", "rb") as f:
    bg_base64 = base64.b64encode(f.read()).decode('utf-8')

response = requests.post(
    "http://localhost:8000/api/slide",
    json={
        "target": f"data:image/png;base64,{target_base64}",
        "background": f"data:image/png;base64,{bg_base64}"
    }
)
result = response.json()
print(f"滑动距离: {result['distance']} 像素")
```

---

### JavaScript 示例

```javascript
// 1. 普通验证码识别
async function recognizeCaptcha(imageBase64) {
    const response = await fetch('http://localhost:8000/api/ocr', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            image: imageBase64,
            type: 'normal'
        })
    });
    
    const data = await response.json();
    console.log('识别结果:', data.result);
    return data.result;
}

// 2. AntiCAP 目标检测
async function detectObjects(imageBase64) {
    const response = await fetch('http://localhost:8000/api/anticap/ocr', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            image: imageBase64,
            type: 'detection_icon'
        })
    });
    
    const data = await response.json();
    console.log('检测结果:', data.result);
    return data.result;
}

// 3. 双图旋转验证码
async function rotateVerify(insideBase64, outsideBase64) {
    const response = await fetch('http://localhost:8000/api/anticap/rotate', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            inside: insideBase64,
            outside: outsideBase64
        })
    });
    
    const data = await response.json();
    console.log('旋转角度:', data.inner_angle);
    return data;
}
```

---

### cURL 示例

```bash
# 1. 健康检查
curl http://localhost:8000/api/health

# 2. 普通验证码识别
curl -X POST http://localhost:8000/api/ocr \
  -H "Content-Type: application/json" \
  -d '{
    "image": "data:image/png;base64,iVBORw0KG...",
    "type": "normal"
  }'

# 3. AntiCAP 算术验证码
curl -X POST http://localhost:8000/api/anticap/ocr \
  -H "Content-Type: application/json" \
  -d '{
    "image": "data:image/png;base64,iVBORw0KG...",
    "type": "math"
  }'

# 4. 查看可用模型
curl http://localhost:8000/api/models
```

---

## 注意事项

1. **Base64 格式**: 支持带或不带 `data:image/png;base64,` 前缀的 Base64 字符串
2. **图片大小**: 建议单张图片不超过 5MB
3. **并发限制**: FastAPI 支持高并发，但建议根据服务器性能合理控制并发数
4. **模型加载**: 首次运行会自动下载模型文件，请确保网络连接正常
5. **超时设置**: 复杂验证码识别可能需要较长时间，建议设置合理的超时时间

---

## 更新日志

### v1.0.0 (2025-12-03)
- ✅ 集成 ddddocr 引擎
- ✅ 集成 AntiCAP 引擎
- ✅ 支持 10+ 种验证码类型
- ✅ 提供完整的 Web Demo
- ✅ 自动 API 文档（Swagger UI）

---

## 技术支持

- **在线文档**: http://localhost:8000/docs (Swagger UI)
- **项目地址**: [GitHub Repository]
- **问题反馈**: [Issues]

---

**最后更新**: 2025-12-03

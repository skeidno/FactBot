# 验证码识别系统更新说明

## 📋 更新概述

已完成验证码识别功能的全面重构，集成了 **ddddocr** 和 **AntiCAP** 两大识别引擎，支持 10+ 种验证码类型。

## 🎯 主要变更

### 后端 API (src/api/captcha/mod.rs)

#### 新增 API 端点

**ddddocr 引擎:**
- `POST /api/captcha/ddddocr/ocr` - 普通验证码识别（标准/旧版/Beta 模式）
- `POST /api/captcha/ddddocr/det` - 目标检测
- `POST /api/captcha/ddddocr/slide` - 滑块验证码
- `POST /api/captcha/ddddocr/click` - 点选验证码

**AntiCAP 引擎:**
- `POST /api/captcha/anticap/ocr` - 通用 OCR（支持 ocr/math/detection_icon/detection_text/single_rotate）
- `POST /api/captcha/anticap/slide` - 滑块验证码（缺口/阴影模式）
- `POST /api/captcha/anticap/rotate` - 双图旋转验证码
- `POST /api/captcha/anticap/compare` - 图片相似度对比

#### 请求格式示例

```json
// ddddocr OCR
{
  "image": "data:image/png;base64,...",
  "type": "normal"  // normal | old | beta
}

// ddddocr 滑块
{
  "target": "data:image/png;base64,...",
  "background": "data:image/png;base64,..."
}

// AntiCAP OCR
{
  "image": "data:image/png;base64,...",
  "type": "ocr"  // ocr | math | detection_icon | detection_text | single_rotate
}

// AntiCAP 滑块
{
  "target": "data:image/png;base64,...",
  "background": "data:image/png;base64,...",
  "mode": "match"  // match | comparison
}
```

#### 响应格式

所有 API 返回统一的 JSON 格式，包含：
- `success`: 是否成功
- `result`: 识别结果（文本/数字/坐标等）
- `objects`: 检测到的目标列表（目标检测）
- `targets`: 可点击目标列表（点选验证码）
- `distance`: 滑动距离（滑块验证码）
- `similarity`: 相似度（图片对比）
- `description`: 描述信息
- `detail`: 错误详情（失败时）

### 前端界面 (src/views/captcha.rs)

#### 新增功能

1. **双引擎选择**
   - ddddocr 引擎：支持 4 种验证码类型
   - AntiCAP 引擎：支持 9 种验证码类型

2. **验证码类型**

   **ddddocr 引擎:**
   - 🔤 普通验证码识别（标准/旧版/Beta 模式）
   - 🎯 目标检测
   - 🧩 滑块验证码
   - 👆 点选验证码

   **AntiCAP 引擎:**
   - 🔤 通用 OCR 识别
   - 🔢 算术验证码
   - 🎨 图标检测
   - 📝 文字检测
   - 🔄 单图旋转
   - 🧩 缺口滑块
   - 👥 阴影滑块
   - 🔄🔄 双图旋转
   - 📊 图片相似度

3. **智能图片上传**
   - 支持主图和副图上传（根据验证码类型自动显示）
   - 实时预览
   - Base64 编码自动处理

4. **结果展示**
   - 成功/失败状态清晰标识
   - 详细的识别结果（文本、坐标、距离、角度等）
   - 格式化的多行显示

## 🔧 技术细节

### 数据结构

```rust
// 引擎枚举
enum CaptchaEngine {
    Ddddocr,
    AntiCAP,
}

// 验证码类型
enum CaptchaType {
    // ddddocr
    DdddocrOcr,
    DdddocrDet,
    DdddocrSlide,
    DdddocrClick,
    
    // AntiCAP
    AnticapOcr,
    AnticapMath,
    AnticapDetIcon,
    AnticapDetText,
    AnticapRotate,
    AnticapSlideMatch,
    AnticapSlideComp,
    AnticapRotateDouble,
    AnticapCompare,
}

// API 响应
struct ApiResponse {
    success: bool,
    result: Option<serde_json::Value>,
    objects: Option<Vec<DetectionObject>>,
    targets: Option<Vec<ClickTarget>>,
    distance: Option<i32>,
    similarity: Option<f64>,
    inner_angle: Option<f64>,
    description: Option<String>,
    detail: Option<String>,
}
```

### 辅助函数

- `decode_base64_image()`: 解码 Base64 图片（支持带/不带前缀）
- `build_request()`: 根据引擎和类型构建请求
- `format_result()`: 格式化 API 响应为可读文本
- `needs_second_image()`: 判断是否需要第二张图片
- `get_second_image_label()`: 获取第二张图片的标签

## 📝 使用示例

### 前端使用

1. 选择识别引擎（ddddocr 或 AntiCAP）
2. 选择验证码类型
3. 上传主图（必需）
4. 上传副图（如果需要，如滑块、旋转等）
5. 点击"开始识别"
6. 查看识别结果

### API 调用示例

```bash
# ddddocr 普通验证码
curl -X POST http://localhost:8080/api/captcha/ddddocr/ocr \
  -H "Content-Type: application/json" \
  -d '{
    "image": "data:image/png;base64,iVBORw0KG...",
    "type": "normal"
  }'

# AntiCAP 算术验证码
curl -X POST http://localhost:8080/api/captcha/anticap/ocr \
  -H "Content-Type: application/json" \
  -d '{
    "image": "data:image/png;base64,iVBORw0KG...",
    "type": "math"
  }'

# ddddocr 滑块验证码
curl -X POST http://localhost:8080/api/captcha/ddddocr/slide \
  -H "Content-Type: application/json" \
  -d '{
    "target": "data:image/png;base64,iVBORw0KG...",
    "background": "data:image/png;base64,iVBORw0KG..."
  }'
```

## ✅ 测试状态

- ✅ 代码编译通过
- ✅ 类型检查通过
- ✅ API 路由注册完成
- ✅ 前端界面重构完成
- ⚠️ 需要运行时测试验证功能

## 📚 参考文档

详细的 API 文档请参考：`API_DOCUMENTATION.md`

## 🎨 UI 改进

- 现代化的渐变背景
- 清晰的引擎选择按钮
- 响应式布局（左右分栏）
- 实时状态反馈
- 美观的结果展示卡片

## 🔄 迁移说明

### 旧接口保留

原有的 `/api/captcha/solve` 接口仍然保留，向后兼容。

### 新接口优势

- 更清晰的 API 结构
- 支持更多验证码类型
- 更详细的响应信息
- 符合 RESTful 规范

---

**更新日期**: 2025-12-03
**版本**: v2.0.0

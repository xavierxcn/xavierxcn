# Design: 占星星盘可视化

## 技术架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Content (Markdown)                        │
│  content/astrology/signs/aries.md                           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ ---                                                  │   │
│  │ title: 白羊座                                        │   │
│  │ chart:                                               │   │
│  │   type: sign                                         │   │
│  │   highlight: aries                                   │   │
│  │ ---                                                  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Generator (Rust)                          │
│  解析 front matter → 传递 chart 配置到模板上下文              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Template (Tera)                           │
│  astrology-item.html                                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ {% if item.chart %}                                  │   │
│  │   <div id="astro-chart"></div>                       │   │
│  │   <script>                                           │   │
│  │     window.chartConfig = {{ item.chart | json }};    │   │
│  │   </script>                                          │   │
│  │ {% endif %}                                          │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Browser (JavaScript)                      │
│  AstroChart 库读取 window.chartConfig 渲染 SVG 星盘          │
└─────────────────────────────────────────────────────────────┘
```

## 数据结构

### Chart 配置 (Rust)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// 星盘类型: sign, planet, house, aspect, full
    #[serde(rename = "type")]
    pub chart_type: String,

    /// 高亮的元素标识
    #[serde(default)]
    pub highlight: Option<String>,

    /// 是否显示宫位线
    #[serde(default = "default_true")]
    pub show_houses: bool,

    /// 是否显示行星
    #[serde(default)]
    pub show_planets: bool,

    /// 示例行星位置（用于演示）
    #[serde(default)]
    pub planets: Option<HashMap<String, f64>>,
}
```

### 星座/行星映射

```javascript
const SIGN_DEGREES = {
  aries: 0, taurus: 30, gemini: 60, cancer: 90,
  leo: 120, virgo: 150, libra: 180, scorpio: 210,
  sagittarius: 240, capricorn: 270, aquarius: 300, pisces: 330
};

const PLANET_SYMBOLS = {
  sun: '☉', moon: '☽', mercury: '☿', venus: '♀',
  mars: '♂', jupiter: '♃', saturn: '♄', uranus: '♅',
  neptune: '♆', pluto: '♇'
};
```

## 模板修改

### astrology-item.html 变更

```html
<!-- 在 item-hero 区域后添加星盘容器 -->
{% if item.chart %}
<div class="chart-container">
    <div id="astro-chart"></div>
</div>
{% endif %}

<!-- 在 block head 中添加 -->
{% if item.chart %}
<script src="https://cdn.jsdelivr.net/npm/@astrodraw/astrochart@3/dist/astrochart.min.js"></script>
<script>
    window.CHART_CONFIG = {
        type: "{{ item.chart.type }}",
        highlight: "{{ item.chart.highlight | default(value='') }}",
        showHouses: {{ item.chart.show_houses | default(value=true) }},
        showPlanets: {{ item.chart.show_planets | default(value=false) }}
    };
</script>
{% endif %}

<!-- 在 block scripts 中添加 -->
{% if item.chart %}
<script src="{{ site.base_path }}/js/astro-chart-renderer.js"></script>
{% endif %}
```

## 渲染脚本

### astro-chart-renderer.js

```javascript
(function() {
    const config = window.CHART_CONFIG;
    if (!config) return;

    const container = document.getElementById('astro-chart');
    if (!container) return;

    // 创建星盘配置
    const chartSettings = {
        SYMBOL_SCALE: 0.8,
        COLOR_BACKGROUND: 'transparent',
        // ... 其他样式配置
    };

    // 根据类型生成示例数据
    const chartData = generateChartData(config);

    // 渲染星盘
    const chart = new astrology.Chart('astro-chart', 400, 400, chartSettings);
    chart.radix(chartData);

    // 高亮指定元素
    if (config.highlight) {
        highlightElement(config.type, config.highlight);
    }
})();
```

## 样式设计

```css
.chart-container {
    display: flex;
    justify-content: center;
    padding: var(--space-xl) 0;
    margin: var(--space-xl) 0;
    border-top: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
}

#astro-chart {
    max-width: 400px;
    width: 100%;
}

#astro-chart svg {
    width: 100%;
    height: auto;
}

/* 高亮样式 */
.chart-highlight {
    fill: var(--color-accent);
    opacity: 0.3;
}
```

## CDN 引用

使用 jsDelivr CDN 加载 AstroChart：

```html
<script src="https://cdn.jsdelivr.net/npm/@astrodraw/astrochart@3/dist/astrochart.min.js"></script>
```

备选方案：将库文件下载到 `themes/default/static/js/` 目录本地托管。

## 降级策略

当 JavaScript 禁用时：
- 星盘容器不显示（使用 `<noscript>` 提示）
- 文章文字内容完整可读
- 符合项目"渐进增强"的设计原则

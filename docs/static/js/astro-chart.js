/**
 * Astrology Chart Renderer using AstrologyChart2
 * 使用 AstrologyChart2 库绘制专业星盘图
 */
(function() {
    'use strict';

    const config = window.CHART_CONFIG;
    if (!config) return;

    const container = document.getElementById('astro-chart');
    if (!container) return;

    if (typeof astrology === 'undefined') {
        console.error('AstrologyChart2 library not loaded');
        return;
    }

    // 星座度数
    const SIGN_DEGREES = {
        aries: 0, taurus: 30, gemini: 60, cancer: 90,
        leo: 120, virgo: 150, libra: 180, scorpio: 210,
        sagittarius: 240, capricorn: 270, aquarius: 300, pisces: 330
    };

    // 星座颜色
    const SIGN_COLORS = {
        aries: '#e74c3c', taurus: '#27ae60', gemini: '#f1c40f', cancer: '#3498db',
        leo: '#e67e22', virgo: '#1abc9c', libra: '#e91e63', scorpio: '#9b59b6',
        sagittarius: '#00bcd4', capricorn: '#607d8b', aquarius: '#2196f3', pisces: '#673ab7'
    };

    const SIGN_ORDER = ['aries', 'taurus', 'gemini', 'cancer', 'leo', 'virgo',
                        'libra', 'scorpio', 'sagittarius', 'capricorn', 'aquarius', 'pisces'];

    // 标准等宫制 (官方示例使用的格式)
    const EQUAL_CUSPS = [
        {angle: 0}, {angle: 30}, {angle: 60}, {angle: 90},
        {angle: 120}, {angle: 150}, {angle: 180}, {angle: 210},
        {angle: 240}, {angle: 270}, {angle: 300}, {angle: 330}
    ];

    // 为每个星座生成独特的星盘数据
    function getSignChartData(signId) {
        const signDegree = SIGN_DEGREES[signId];
        const signIndex = SIGN_ORDER.indexOf(signId);

        // 太阳在该星座中间，其他行星按季节分布
        const points = [
            { name: 'Sun', angle: signDegree + 15 },
            { name: 'Moon', angle: (signDegree + 120 + signIndex * 25) % 360 },
            { name: 'Mercury', angle: (signDegree + signIndex * 3) % 360 },  // 水星靠近太阳
            { name: 'Venus', angle: (signDegree + 25 + signIndex * 5) % 360 }, // 金星靠近太阳
            { name: 'Mars', angle: (signIndex * 30 + 180) % 360 },
            { name: 'Jupiter', angle: (signIndex * 30 + 60) % 360 },
            { name: 'Saturn', angle: (signIndex * 30 + 240) % 360, isRetrograde: signIndex % 3 === 0 },
            { name: 'Uranus', angle: (40 + signIndex * 8) % 360 },
            { name: 'Neptune', angle: (355 + signIndex * 2) % 360, isRetrograde: true },
            { name: 'Pluto', angle: (295 + signIndex * 3) % 360 },
            { name: 'Lilith', angle: (200 + signIndex * 15) % 360 },
            { name: 'Chiron', angle: (320 + signIndex * 10) % 360 },
            { name: 'NNode', angle: (280 + signIndex * 12) % 360 }
        ];

        return { points, cusps: EQUAL_CUSPS };
    }

    // 为每个行星生成独特配置
    function getPlanetChartData(planetId) {
        // 行星庙位
        const planetDomicile = {
            sun: 120, moon: 90, mercury: 150, venus: 30, mars: 0,
            jupiter: 240, saturn: 270, uranus: 300, neptune: 330, pluto: 210
        };

        const targetAngle = (planetDomicile[planetId] || 0) + 15;

        const basePoints = [
            { name: 'Sun', angle: 273 },
            { name: 'Moon', angle: 64 },
            { name: 'Mercury', angle: 194 },
            { name: 'Venus', angle: 158 },
            { name: 'Mars', angle: 304 },
            { name: 'Jupiter', angle: 229 },
            { name: 'Saturn', angle: 255, isRetrograde: true },
            { name: 'Uranus', angle: 347 },
            { name: 'Neptune', angle: 21, isRetrograde: true },
            { name: 'Pluto', angle: 233 },
            { name: 'Lilith', angle: 244 },
            { name: 'Chiron', angle: 339 },
            { name: 'NNode', angle: 285 }
        ];

        // 将目标行星移到其庙位
        const planetNameMap = {
            sun: 'Sun', moon: 'Moon', mercury: 'Mercury', venus: 'Venus',
            mars: 'Mars', jupiter: 'Jupiter', saturn: 'Saturn',
            uranus: 'Uranus', neptune: 'Neptune', pluto: 'Pluto'
        };

        const points = basePoints.map(p => {
            if (p.name === planetNameMap[planetId]) {
                return { ...p, angle: targetAngle };
            }
            return p;
        });

        return { points, cusps: EQUAL_CUSPS };
    }

    // 为每个宫位生成配置
    function getHouseChartData(houseNum) {
        const houseIndex = parseInt(houseNum) - 1;
        const houseMiddle = houseIndex * 30 + 15;

        // 在目标宫位放置多颗行星
        const points = [
            { name: 'Sun', angle: houseMiddle },
            { name: 'Moon', angle: houseMiddle + 10 },
            { name: 'Mercury', angle: houseMiddle + 5 },
            { name: 'Venus', angle: (houseMiddle + 60) % 360 },
            { name: 'Mars', angle: (houseMiddle + 120) % 360 },
            { name: 'Jupiter', angle: (houseMiddle + 180) % 360 },
            { name: 'Saturn', angle: (houseMiddle + 240) % 360, isRetrograde: true },
            { name: 'Uranus', angle: (houseMiddle + 90) % 360 },
            { name: 'Neptune', angle: (houseMiddle + 270) % 360, isRetrograde: true },
            { name: 'Pluto', angle: (houseMiddle + 150) % 360 },
            { name: 'Lilith', angle: (houseMiddle + 200) % 360 },
            { name: 'Chiron', angle: (houseMiddle + 300) % 360 },
            { name: 'NNode', angle: (houseMiddle + 330) % 360 }
        ];

        return { points, cusps: EQUAL_CUSPS };
    }

    // 为相位生成配置
    function getAspectChartData(aspectId) {
        const aspectAngles = {
            conjunction: 0,
            sextile: 60,
            square: 90,
            trine: 120,
            opposition: 180
        };

        const angle = aspectAngles[aspectId] || 0;
        const baseAngle = 45;

        // 太阳和月亮形成指定相位
        const points = [
            { name: 'Sun', angle: baseAngle },
            { name: 'Moon', angle: (baseAngle + angle) % 360 },
            { name: 'Mercury', angle: 194 },
            { name: 'Venus', angle: 158 },
            { name: 'Mars', angle: 304 },
            { name: 'Jupiter', angle: 229 },
            { name: 'Saturn', angle: 255, isRetrograde: true },
            { name: 'Uranus', angle: 347 },
            { name: 'Neptune', angle: 21, isRetrograde: true },
            { name: 'Pluto', angle: 233 },
            { name: 'Lilith', angle: 244 },
            { name: 'Chiron', angle: 339 },
            { name: 'NNode', angle: 285 }
        ];

        return { points, cusps: EQUAL_CUSPS };
    }

    // 基础知识/完整星盘
    function getFullChartData() {
        const points = [
            { name: 'Sun', angle: 273 },
            { name: 'Moon', angle: 64 },
            { name: 'Mercury', angle: 194 },
            { name: 'Venus', angle: 158 },
            { name: 'Mars', angle: 304 },
            { name: 'Jupiter', angle: 229 },
            { name: 'Saturn', angle: 255, isRetrograde: true },
            { name: 'Uranus', angle: 347 },
            { name: 'Neptune', angle: 21, isRetrograde: true },
            { name: 'Pluto', angle: 233 },
            { name: 'Lilith', angle: 244 },
            { name: 'Chiron', angle: 339 },
            { name: 'NNode', angle: 285 }
        ];
        return { points, cusps: EQUAL_CUSPS };
    }

    // 生成高亮星座的设置
    function getSignHighlightSettings(highlightSign) {
        const settings = {};
        SIGN_ORDER.forEach(sign => {
            const colorKey = 'COLOR_' + sign.toUpperCase();
            settings[colorKey] = (sign === highlightSign) ? SIGN_COLORS[sign] : '#e8e8e8';
        });
        return settings;
    }

    // 获取图表配置
    function getChartConfig() {
        let data, settings = {};

        if (config.type === 'sign' && config.highlight) {
            data = getSignChartData(config.highlight);
            settings = getSignHighlightSettings(config.highlight);
        } else if (config.type === 'planet' && config.highlight) {
            data = getPlanetChartData(config.highlight);
        } else if (config.type === 'house' && config.highlight) {
            data = getHouseChartData(config.highlight);
        } else if (config.type === 'aspect' && config.highlight) {
            data = getAspectChartData(config.highlight);
        } else {
            data = getFullChartData();
        }

        return { data, settings };
    }

    // 创建图表
    try {
        const { data, settings } = getChartConfig();

        // 创建星盘并绘制相位
        const radix = new astrology.Universe('astro-chart', settings).radix().setData(data);
        radix.drawAspects();

    } catch (error) {
        console.error('Error creating chart:', error);
        container.innerHTML = '<div style="text-align:center;color:#888;padding:40px;">星盘渲染错误: ' + error.message + '</div>';
    }
})();

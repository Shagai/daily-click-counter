use crate::models::DayCounts;

pub fn render_index(date: &str, counts: &DayCounts) -> String {
    let net = counts.add as i64 - counts.sub as i64;
    INDEX_HTML
        .replace("{{DATE}}", date)
        .replace("{{ADD}}", &counts.add.to_string())
        .replace("{{SUB}}", &counts.sub.to_string())
        .replace("{{NET}}", &net.to_string())
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Daily Click Counter</title>
  <style>
    @import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600&family=Fraunces:wght@600&display=swap');

    :root {
      --bg-1: #f8f3e6;
      --bg-2: #f5d3a7;
      --ink: #2b2a28;
      --accent: #ff6b4a;
      --accent-2: #2f4858;
      --card: rgba(255, 255, 255, 0.86);
      --shadow: 0 24px 60px rgba(47, 72, 88, 0.18);
    }

    * {
      box-sizing: border-box;
    }

    body {
      margin: 0;
      min-height: 100vh;
      background: radial-gradient(circle at top, var(--bg-2), transparent 60%),
        linear-gradient(135deg, var(--bg-1), #ffe9d4 60%, #f9f2e9 100%);
      color: var(--ink);
      font-family: "Space Grotesk", "Trebuchet MS", sans-serif;
      display: grid;
      place-items: center;
      padding: 32px 18px 48px;
    }

    .app {
      width: min(860px, 100%);
      background: var(--card);
      backdrop-filter: blur(12px);
      border-radius: 28px;
      box-shadow: var(--shadow);
      padding: 36px;
      display: grid;
      gap: 28px;
      animation: rise 600ms ease;
    }

    header {
      display: flex;
      flex-direction: column;
      gap: 6px;
    }

    h1 {
      font-family: "Fraunces", "Georgia", serif;
      font-weight: 600;
      font-size: clamp(2rem, 4vw, 2.8rem);
      margin: 0;
    }

    .subtitle {
      margin: 0;
      color: #5f5c57;
      font-size: 1rem;
    }

    .panel {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
      gap: 16px;
    }

    .stat {
      background: white;
      border-radius: 18px;
      padding: 18px;
      border: 1px solid rgba(47, 72, 88, 0.08);
      display: grid;
      gap: 8px;
    }

    .stat span {
      display: block;
    }

    .stat .label {
      font-size: 0.85rem;
      text-transform: uppercase;
      letter-spacing: 0.12em;
      color: #8b857d;
    }

    .stat .value {
      font-size: 1.7rem;
      font-weight: 600;
      color: var(--accent-2);
    }

    .stat .value.net {
      color: var(--accent);
    }

    .actions {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 16px;
    }

    button {
      appearance: none;
      border: none;
      border-radius: 999px;
      padding: 16px 20px;
      font-size: 1rem;
      font-weight: 600;
      cursor: pointer;
      transition: transform 150ms ease, box-shadow 150ms ease;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
    }

    button:active {
      transform: scale(0.98);
    }

    .btn-add {
      background: var(--accent);
      color: white;
      box-shadow: 0 10px 24px rgba(255, 107, 74, 0.3);
    }

    .btn-sub {
      background: var(--accent-2);
      color: white;
      box-shadow: 0 10px 24px rgba(47, 72, 88, 0.3);
    }

    .chart-area {
      display: grid;
      gap: 16px;
    }

    .chart-header {
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: space-between;
      gap: 16px;
    }

    .chart-header h2 {
      margin: 0;
      font-size: 1.4rem;
    }

    .chart-header .subtitle {
      margin-top: 6px;
      font-size: 0.95rem;
    }

    .tabs {
      display: flex;
      gap: 6px;
      padding: 6px;
      background: rgba(47, 72, 88, 0.08);
      border-radius: 999px;
    }

    .tab {
      background: transparent;
      border: none;
      border-radius: 999px;
      padding: 8px 14px;
      font-size: 0.9rem;
      font-weight: 600;
      color: #6b645d;
      box-shadow: none;
    }

    .tab.active {
      background: white;
      color: var(--accent-2);
      box-shadow: 0 8px 16px rgba(47, 72, 88, 0.12);
    }

    .chart-card {
      background: white;
      border-radius: 20px;
      padding: 16px;
      border: 1px solid rgba(47, 72, 88, 0.08);
    }

    #chart {
      width: 100%;
      height: 260px;
      display: block;
    }

    #chart text {
      font-family: "Space Grotesk", "Trebuchet MS", sans-serif;
    }

    .chart-line {
      fill: none;
      stroke: var(--accent);
      stroke-width: 3;
    }

    .chart-point {
      fill: white;
      stroke: var(--accent);
      stroke-width: 2;
    }

    .chart-grid {
      stroke: rgba(47, 72, 88, 0.12);
    }

    .chart-axis {
      stroke: rgba(47, 72, 88, 0.25);
      stroke-dasharray: 4 6;
    }

    .chart-label {
      fill: #7a746d;
      font-size: 11px;
    }

    .chart-metrics {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
      gap: 16px;
    }

    .status {
      font-size: 0.95rem;
      color: #6b645d;
      min-height: 1.2em;
    }

    .status[data-type="error"] {
      color: #c63b2b;
    }

    .status[data-type="ok"] {
      color: #2d7a4b;
    }

    .hint {
      margin: 0;
      color: #6f6a65;
      font-size: 0.9rem;
    }

    @keyframes rise {
      from {
        opacity: 0;
        transform: translateY(18px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    @media (max-width: 600px) {
      .app {
        padding: 28px 22px;
      }
      button {
        width: 100%;
      }
    }
  </style>
</head>
<body>
  <main class="app">
    <header>
      <h1>Daily Click Counter</h1>
      <p class="subtitle">Track adds and subtracts for each day, then build stats panels later.</p>
    </header>

    <section class="panel">
      <div class="stat">
        <span class="label">Date</span>
        <span id="date" class="value">{{DATE}}</span>
      </div>
      <div class="stat">
        <span class="label">Adds</span>
        <span id="adds" class="value">{{ADD}}</span>
      </div>
      <div class="stat">
        <span class="label">Subtracts</span>
        <span id="subs" class="value">{{SUB}}</span>
      </div>
      <div class="stat">
        <span class="label">Net</span>
        <span id="net" class="value net">{{NET}}</span>
      </div>
    </section>

    <section class="actions">
      <form id="add-form" method="post" action="/click/add">
        <button class="btn-add" id="add-btn" type="submit">Add +1</button>
      </form>
      <form id="sub-form" method="post" action="/click/sub">
        <button class="btn-sub" id="sub-btn" type="submit">Subtract -1</button>
      </form>
    </section>

    <section class="chart-area">
      <div class="chart-header">
        <div>
          <h2 id="chart-title">Last 7 days</h2>
          <p id="chart-subtitle" class="subtitle">Net change (adds - subtracts).</p>
        </div>
        <div class="tabs" role="tablist">
          <button class="tab active" type="button" data-tab="daily" role="tab" aria-selected="true">Last 7 days</button>
          <button class="tab" type="button" data-tab="weekly" role="tab" aria-selected="false">Weekly totals</button>
          <button class="tab" type="button" data-tab="average" role="tab" aria-selected="false">Weekly averages</button>
        </div>
      </div>
      <div class="chart-card">
        <svg id="chart" viewBox="0 0 600 260" aria-label="Stats chart" role="img"></svg>
      </div>
      <div class="chart-metrics">
        <div class="stat">
          <span class="label" id="metric-1-label">Total adds</span>
          <span class="value" id="metric-1-value">0</span>
        </div>
        <div class="stat">
          <span class="label" id="metric-2-label">Total subtracts</span>
          <span class="value" id="metric-2-value">0</span>
        </div>
        <div class="stat">
          <span class="label" id="metric-3-label">Net</span>
          <span class="value net" id="metric-3-value">0</span>
        </div>
      </div>
    </section>

    <div class="status" id="status"></div>
    <p class="hint">Counts are kept per calendar day (server time). Weekly averages are per day; the current week uses days so far.</p>
  </main>

  <script>
    const dateEl = document.getElementById('date');
    const addsEl = document.getElementById('adds');
    const subsEl = document.getElementById('subs');
    const netEl = document.getElementById('net');
    const statusEl = document.getElementById('status');
    const chartEl = document.getElementById('chart');
    const chartTitleEl = document.getElementById('chart-title');
    const chartSubtitleEl = document.getElementById('chart-subtitle');
    const metric1Label = document.getElementById('metric-1-label');
    const metric1Value = document.getElementById('metric-1-value');
    const metric2Label = document.getElementById('metric-2-label');
    const metric2Value = document.getElementById('metric-2-value');
    const metric3Label = document.getElementById('metric-3-label');
    const metric3Value = document.getElementById('metric-3-value');
    const tabs = Array.from(document.querySelectorAll('.tab'));

    let statsData = null;
    let activeTab = 'daily';

    const setStatus = (message, type) => {
      statusEl.textContent = message;
      statusEl.dataset.type = type || '';
    };

    const updateUI = (data) => {
      dateEl.textContent = data.date;
      addsEl.textContent = data.add_count;
      subsEl.textContent = data.sub_count;
      netEl.textContent = data.net;
    };

    const formatMetric = (value, decimals = 0) => {
      if (typeof value !== 'number' || Number.isNaN(value)) {
        return '--';
      }
      const factor = Math.pow(10, decimals);
      const rounded = Math.round(value * factor) / factor;
      if (decimals === 0) {
        return Math.round(rounded).toString();
      }
      return rounded.toFixed(decimals).replace(/\.0+$/, '');
    };

    const formatAxisValue = (value) => {
      const rounded = Math.round(value * 10) / 10;
      return Number.isInteger(rounded) ? rounded.toString() : rounded.toFixed(1);
    };

    const renderLineChart = (points) => {
      if (!points.length) {
        chartEl.innerHTML = '<text class="chart-label" x="50%" y="50%" text-anchor="middle">No data yet</text>';
        return;
      }

      const width = 600;
      const height = 260;
      const paddingX = 44;
      const paddingY = 34;
      const top = 24;

      const values = points.map((point) => point.value);
      let min = Math.min(...values);
      let max = Math.max(...values);
      min = Math.min(min, 0);
      max = Math.max(max, 0);
      if (min === max) {
        min -= 1;
        max += 1;
      }

      const range = max - min;
      const xStep = points.length > 1 ? (width - paddingX * 2) / (points.length - 1) : 0;
      const scaleY = (height - top - paddingY) / range;
      const x = (index) => paddingX + index * xStep;
      const y = (value) => height - paddingY - (value - min) * scaleY;

      const path = points
        .map((point, index) => `${index === 0 ? 'M' : 'L'} ${x(index).toFixed(2)} ${y(point.value).toFixed(2)}`)
        .join(' ');

      const ticks = 4;
      let grid = '';
      for (let i = 0; i <= ticks; i += 1) {
        const value = min + (range * i) / ticks;
        const yPos = y(value);
        grid += `<line class="chart-grid" x1="${paddingX}" y1="${yPos}" x2="${width - paddingX}" y2="${yPos}" />`;
        grid += `<text class="chart-label" x="${paddingX - 10}" y="${yPos + 4}" text-anchor="end">${formatAxisValue(value)}</text>`;
      }

      const labelEvery = points.length > 8 ? 2 : 1;
      const xLabels = points
        .map((point, index) => {
          if (index % labelEvery !== 0) {
            return '';
          }
          return `<text class="chart-label" x="${x(index)}" y="${height - paddingY + 18}" text-anchor="middle">${point.label}</text>`;
        })
        .join('');

      const circles = points
        .map((point, index) => `<circle class="chart-point" cx="${x(index)}" cy="${y(point.value)}" r="4" />`)
        .join('');

      const zeroLine = `<line class="chart-axis" x1="${paddingX}" y1="${y(0)}" x2="${width - paddingX}" y2="${y(0)}" />`;

      chartEl.setAttribute('viewBox', `0 0 ${width} ${height}`);
      chartEl.innerHTML = `
        ${grid}
        ${zeroLine}
        <path class="chart-line" d="${path}" />
        ${circles}
        ${xLabels}
      `;
    };

    const setMetrics = (items) => {
      const [first, second, third] = items;
      metric1Label.textContent = first.label;
      metric1Value.textContent = formatMetric(first.value, first.decimals || 0);
      metric2Label.textContent = second.label;
      metric2Value.textContent = formatMetric(second.value, second.decimals || 0);
      metric3Label.textContent = third.label;
      metric3Value.textContent = formatMetric(third.value, third.decimals || 0);
    };

    const renderDaily = () => {
      const points = statsData.last_7_days.map((day) => ({
        label: day.date.slice(5),
        value: day.net
      }));
      const totals = statsData.last_7_days.reduce(
        (acc, day) => ({
          add: acc.add + day.add_count,
          sub: acc.sub + day.sub_count
        }),
        { add: 0, sub: 0 }
      );
      chartTitleEl.textContent = 'Last 7 days';
      chartSubtitleEl.textContent = 'Net change (adds - subtracts).';
      renderLineChart(points);
      setMetrics([
        { label: 'Total adds', value: totals.add },
        { label: 'Total subtracts', value: totals.sub },
        { label: 'Net', value: totals.add - totals.sub }
      ]);
    };

    const renderWeeklyTotals = () => {
      const points = statsData.weekly_totals.map((week) => ({
        label: week.week,
        value: week.net
      }));
      const current = statsData.weekly_totals[statsData.weekly_totals.length - 1];
      chartTitleEl.textContent = 'Weekly totals';
      chartSubtitleEl.textContent = `Totals for ${current.start_date} → ${current.end_date}.`;
      renderLineChart(points);
      setMetrics([
        { label: 'This week adds', value: current.add_count },
        { label: 'This week subtracts', value: current.sub_count },
        { label: 'This week net', value: current.net }
      ]);
    };

    const renderWeeklyAverages = () => {
      const points = statsData.weekly_averages.map((week) => ({
        label: week.week,
        value: week.avg_net
      }));
      const current = statsData.weekly_averages[statsData.weekly_averages.length - 1];
      chartTitleEl.textContent = 'Weekly averages';
      chartSubtitleEl.textContent = `Average per day (current week: ${current.days_counted} days).`;
      renderLineChart(points);
      setMetrics([
        { label: 'Avg adds/day', value: current.avg_add, decimals: 1 },
        { label: 'Avg subtracts/day', value: current.avg_sub, decimals: 1 },
        { label: 'Avg net/day', value: current.avg_net, decimals: 1 }
      ]);
    };

    const renderActiveTab = () => {
      if (!statsData) {
        return;
      }
      if (activeTab === 'weekly') {
        renderWeeklyTotals();
      } else if (activeTab === 'average') {
        renderWeeklyAverages();
      } else {
        renderDaily();
      }
    };

    const setActiveTab = (tab) => {
      activeTab = tab;
      tabs.forEach((button) => {
        const isActive = button.dataset.tab === tab;
        button.classList.toggle('active', isActive);
        button.setAttribute('aria-selected', String(isActive));
      });
      renderActiveTab();
    };

    const loadToday = async () => {
      const res = await fetch('/api/today');
      if (!res.ok) {
        throw new Error('Unable to load today data');
      }
      updateUI(await res.json());
    };

    const loadStats = async () => {
      const res = await fetch('/api/stats');
      if (!res.ok) {
        throw new Error('Unable to load stats');
      }
      statsData = await res.json();
      renderActiveTab();
    };

    const refresh = async () => {
      await Promise.all([loadToday(), loadStats()]);
    };

    const send = async (action) => {
      setStatus('Saving...', 'info');
      const res = await fetch('/api/click', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ action })
      });

      if (!res.ok) {
        const msg = await res.text();
        throw new Error(msg || 'Request failed');
      }

      updateUI(await res.json());
      loadStats().catch((err) => setStatus(err.message, 'error'));
      setStatus('Saved', 'ok');
      setTimeout(() => setStatus('', ''), 1200);
    };

    tabs.forEach((button) => {
      button.addEventListener('click', () => setActiveTab(button.dataset.tab));
    });

    const addForm = document.getElementById('add-form');
    const subForm = document.getElementById('sub-form');

    addForm.addEventListener('submit', (event) => {
      event.preventDefault();
      send('add').catch((err) => setStatus(err.message, 'error'));
    });

    subForm.addEventListener('submit', (event) => {
      event.preventDefault();
      send('sub').catch((err) => setStatus(err.message, 'error'));
    });

    refresh().catch((err) => setStatus(err.message, 'error'));
  </script>
</body>
</html>
"#;

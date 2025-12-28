<script lang="ts">
	import { onMount } from "svelte";

	let mainPanelWidth = 70; // percentage
	let bottomPanelHeight = 30; // percentage
	let isResizingVertical = false;
	let isResizingHorizontal = false;

	function startVerticalResize() {
		isResizingVertical = true;
	}

	function startHorizontalResize() {
		isResizingHorizontal = true;
	}

	onMount(() => {
		const handleMouseMove = (e: MouseEvent) => {
			if (isResizingVertical) {
				const container = document.querySelector(".dashboard-content");
				if (container) {
					const rect = container.getBoundingClientRect();
					const newWidth = ((e.clientX - rect.left) / rect.width) * 100;
					if (newWidth > 15 && newWidth < 85) {
						mainPanelWidth = newWidth;
					}
				}
			}
			if (isResizingHorizontal) {
				const container = document.querySelector(".main-area");
				if (container) {
					const rect = container.getBoundingClientRect();
					const newHeight = ((rect.bottom - e.clientY) / rect.height) * 100;
					if (newHeight > 15 && newHeight < 60) {
						bottomPanelHeight = newHeight;
					}
				}
			}
		};

		const handleMouseUp = () => {
			isResizingVertical = false;
			isResizingHorizontal = false;
		};

		window.addEventListener("mousemove", handleMouseMove);
		window.addEventListener("mouseup", handleMouseUp);

		return () => {
			window.removeEventListener("mousemove", handleMouseMove);
			window.removeEventListener("mouseup", handleMouseUp);
		};
	});
</script>

<div class="dashboard">
	<!-- Top Toolbar -->
	<div class="toolbar">
		<div class="toolbar-section">
			<button class="toolbar-btn" title="New Chart">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
					/>
					<polyline points="14 2 14 8 20 8" />
					<line x1="12" y1="18" x2="12" y2="12" />
					<line x1="9" y1="15" x2="15" y2="15" />
				</svg>
			</button>
			<button class="toolbar-btn" title="Open">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"
					/>
				</svg>
			</button>
			<button class="toolbar-btn" title="Save">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"
					/>
					<polyline points="17 21 17 13 7 13 7 21" />
					<polyline points="7 3 7 8 15 8" />
				</svg>
			</button>
			<div class="toolbar-separator"></div>
			<button class="toolbar-btn" title="Charts">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<line x1="12" y1="20" x2="12" y2="10" />
					<line x1="18" y1="20" x2="18" y2="4" />
					<line x1="6" y1="20" x2="6" y2="16" />
				</svg>
			</button>
			<button class="toolbar-btn active" title="Candlesticks">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<line x1="9" y1="2" x2="9" y2="22" />
					<line x1="15" y1="2" x2="15" y2="22" />
					<rect x="7" y="6" width="4" height="8" />
					<rect x="13" y="10" width="4" height="6" />
				</svg>
			</button>
			<button class="toolbar-btn" title="Line Chart">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
				</svg>
			</button>
		</div>

		<div class="toolbar-section">
			<select class="toolbar-select">
				<option>EURUSD</option>
				<option>GBPUSD</option>
				<option>USDJPY</option>
			</select>
			<select class="toolbar-select">
				<option>M1</option>
				<option>M15</option>
				<option>M5</option>
				<option>M30</option>
				<option>H1</option>
				<option>H4</option>
				<option>D1</option>
			</select>
		</div>

		<div class="toolbar-section">
			<button class="toolbar-btn" title="Zoom In">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="11" cy="11" r="8" />
					<path d="m21 21-4.35-4.35" />
					<line x1="11" y1="8" x2="11" y2="14" />
					<line x1="8" y1="11" x2="14" y2="11" />
				</svg>
			</button>
			<button class="toolbar-btn" title="Zoom Out">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="11" cy="11" r="8" />
					<path d="m21 21-4.35-4.35" />
					<line x1="8" y1="11" x2="14" y2="11" />
				</svg>
			</button>
		</div>
	</div>

	<!-- Main Content -->
	<div class="dashboard-content">
		<!-- Left Sidebar -->
		<aside class="left-panel">
			<div class="panel-tabs">
				<button class="tab-btn active">Market Watch</button>
				<button class="tab-btn">Navigator</button>
			</div>
			<div class="panel-content">
				<div class="market-item">
					<div class="symbol">EURUSD</div>
					<div class="price bid">1.0845</div>
					<div class="price ask">1.0847</div>
				</div>
				<div class="market-item">
					<div class="symbol">GBPUSD</div>
					<div class="price bid">1.2634</div>
					<div class="price ask">1.2636</div>
				</div>
				<div class="market-item">
					<div class="symbol">USDJPY</div>
					<div class="price bid">149.82</div>
					<div class="price ask">149.85</div>
				</div>
				<div class="market-item">
					<div class="symbol">AUDUSD</div>
					<div class="price bid">0.6521</div>
					<div class="price ask">0.6523</div>
				</div>
			</div>
		</aside>

		<!-- Vertical Resizer -->
		<div class="resizer vertical" on:mousedown={startVerticalResize}></div>

		<!-- Main Area (Chart + Bottom Panel) -->
		<div class="main-area">
			<!-- Chart Panel -->
			<div class="chart-panel" style="height: {100 - bottomPanelHeight}%;">
				<div class="chart-header">
					<span class="chart-title">EURUSD, M15</span>
					<div class="chart-info">
						<span class="info-item">O: 1.0845</span>
						<span class="info-item">H: 1.0852</span>
						<span class="info-item">L: 1.0841</span>
						<span class="info-item">C: 1.0847</span>
					</div>
				</div>
				<div class="chart-content">
					<div class="chart-placeholder">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="64"
							height="64"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1"
						>
							<line x1="12" y1="20" x2="12" y2="10" />
							<line x1="18" y1="20" x2="18" y2="4" />
							<line x1="6" y1="20" x2="6" y2="16" />
						</svg>
						<p>Chart Area</p>
					</div>
				</div>
			</div>

			<!-- Horizontal Resizer -->
			<div
				class="resizer horizontal"
				on:mousedown={startHorizontalResize}
			></div>

			<!-- Bottom Panel -->
			<div class="bottom-panel" style="height: {bottomPanelHeight}%;">
				<div class="panel-tabs">
					<button class="tab-btn active">Terminal</button>
					<button class="tab-btn">Strategy Tester</button>
					<button class="tab-btn">Alerts</button>
				</div>
				<div class="terminal-content">
					<table class="terminal-table">
						<thead>
							<tr>
								<th>Order</th>
								<th>Time</th>
								<th>Symbol</th>
								<th>Type</th>
								<th>Volume</th>
								<th>Price</th>
								<th>S/L</th>
								<th>T/P</th>
								<th>Profit</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td colspan="9" class="no-data">No open positions</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background-color: var(--bg-primary);
		overflow: hidden;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 4px 8px;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		height: 36px;
		flex-shrink: 0;
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 3px;
		color: var(--text-primary);
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.toolbar-btn:hover {
		background-color: var(--border-color);
	}

	.toolbar-btn.active {
		background-color: var(--accent);
		color: white;
	}

	.toolbar-separator {
		width: 1px;
		height: 20px;
		background-color: var(--border-color);
		margin: 0 4px;
	}

	.toolbar-select {
		height: 26px;
		padding: 0 8px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 3px;
		color: var(--text-primary);
		font-size: 12px;
		cursor: pointer;
	}

	/* Dashboard Content */
	.dashboard-content {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	/* Left Panel */
	.left-panel {
		width: 220px;
		background-color: var(--bg-secondary);
		border-right: 1px solid var(--border-color);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
	}

	.panel-tabs {
		display: flex;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
	}

	.tab-btn {
		flex: 1;
		padding: 8px 12px;
		background: transparent;
		border: none;
		color: var(--text-secondary);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab-btn.active {
		color: var(--text-primary);
		background-color: var(--surface-color);
		border-bottom: 2px solid var(--accent);
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 4px;
	}

	.market-item {
		display: grid;
		grid-template-columns: 1fr auto auto;
		gap: 8px;
		padding: 6px 8px;
		font-size: 11px;
		border-bottom: 1px solid var(--border-color);
		cursor: pointer;
	}

	.market-item:hover {
		background-color: var(--border-color);
	}

	.symbol {
		font-weight: 600;
		color: var(--text-primary);
	}

	.price {
		font-family: "Consolas", "Monaco", monospace;
		font-size: 11px;
	}

	.price.bid {
		color: #ef4444;
	}

	.price.ask {
		color: #22c55e;
	}

	/* Resizers */
	.resizer {
		background-color: var(--border-color);
		cursor: ew-resize;
		flex-shrink: 0;
	}

	.resizer.vertical {
		width: 4px;
		cursor: ew-resize;
	}

	.resizer.vertical:hover {
		background-color: var(--accent);
	}

	.resizer.horizontal {
		height: 4px;
		cursor: ns-resize;
	}

	.resizer.horizontal:hover {
		background-color: var(--accent);
	}

	/* Main Area */
	.main-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	/* Chart Panel */
	.chart-panel {
		display: flex;
		flex-direction: column;
		background-color: var(--bg-primary);
		overflow: hidden;
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 4px 12px;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		font-size: 11px;
	}

	.chart-title {
		font-weight: 600;
		color: var(--text-primary);
	}

	.chart-info {
		display: flex;
		gap: 16px;
	}

	.info-item {
		color: var(--text-secondary);
		font-family: "Consolas", "Monaco", monospace;
	}

	.chart-content {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: var(--bg-primary);
	}

	.chart-placeholder {
		text-align: center;
		color: var(--text-secondary);
		opacity: 0.5;
	}

	.chart-placeholder p {
		margin-top: 12px;
		font-size: 13px;
	}

	/* Bottom Panel */
	.bottom-panel {
		display: flex;
		flex-direction: column;
		background-color: var(--bg-secondary);
		border-top: 1px solid var(--border-color);
		overflow: hidden;
	}

	.terminal-content {
		flex: 1;
		overflow: auto;
	}

	.terminal-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.terminal-table thead {
		background-color: var(--bg-secondary);
		position: sticky;
		top: 0;
	}

	.terminal-table th {
		padding: 8px 12px;
		text-align: left;
		font-weight: 600;
		color: var(--text-primary);
		border-bottom: 1px solid var(--border-color);
	}

	.terminal-table td {
		padding: 6px 12px;
		color: var(--text-secondary);
		border-bottom: 1px solid var(--border-color);
	}

	.no-data {
		text-align: center;
		padding: 24px !important;
		color: var(--text-secondary);
	}

	/* Scrollbars */
	.panel-content::-webkit-scrollbar,
	.terminal-content::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}

	.panel-content::-webkit-scrollbar-track,
	.terminal-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.panel-content::-webkit-scrollbar-thumb,
	.terminal-content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>

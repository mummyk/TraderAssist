<script lang="ts">
	export let symbol = "EURUSD";
	export let timeframe = "M15";
	export let ohlc = {
		open: "1.0845",
		high: "1.0852",
		low: "1.0841",
		close: "1.0847",
	};
</script>

<div class="chart-panel">
	<div class="chart-header">
		<span class="chart-title">{symbol}, {timeframe}</span>
		<div class="chart-info">
			<span class="info-item">O: {ohlc.open}</span>
			<span class="info-item">H: {ohlc.high}</span>
			<span class="info-item">L: {ohlc.low}</span>
			<span class="info-item">C: {ohlc.close}</span>
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
			<span class="chart-hint">Chart will be rendered here</span>
		</div>
	</div>
</div>

<style>
	.chart-panel {
		display: flex;
		flex-direction: column;
		background-color: var(--bg-primary);
		overflow: hidden;
		height: 100%;
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 4px 12px;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		font-size: 11px;
		flex-shrink: 0;
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
		overflow: hidden;
	}

	.chart-placeholder {
		text-align: center;
		color: var(--text-secondary);
		opacity: 0.5;
	}

	.chart-placeholder p {
		margin-top: 12px;
		font-size: 13px;
		font-weight: 500;
	}

	.chart-hint {
		display: block;
		margin-top: 4px;
		font-size: 11px;
	}
</style>

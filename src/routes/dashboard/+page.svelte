<script lang="ts">
	import { onMount } from "svelte";
	import Toolbar from "$lib/components/dashboard/Toolbar.svelte";
	import MarketWatch from "$lib/components/dashboard/MarketWatch.svelte";
	import ChartPanel from "$lib/components/dashboard/ChartPanel.svelte";
	import Terminal from "$lib/components/dashboard/Terminal.svelte";
	import Resizer from "$lib/components/dashboard/Resizer.svelte";

	let selectedSymbol = "EURUSD";
	let selectedTimeframe = "M15";
	let chartType = "candlesticks";
	let isPlaying = false;
	let isPaused = false;
	let playbackSpeed = 1;

	let mainPanelWidth = 70;
	let bottomPanelHeight = 30;
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
	<Toolbar
		bind:selectedSymbol
		bind:selectedTimeframe
		bind:chartType
		bind:isPlaying
		bind:isPaused
		bind:playbackSpeed
	/>

	<div class="dashboard-content">
		<MarketWatch />

		<Resizer direction="vertical" onResize={startVerticalResize} />

		<div class="main-area">
			<div class="chart-container" style="height: {100 - bottomPanelHeight}%;">
				<ChartPanel
					symbol={selectedSymbol}
					timeframe={selectedTimeframe}
					ohlc={{
						open: "1.0845",
						high: "1.0852",
						low: "1.0841",
						close: "1.0847",
					}}
				/>
			</div>

			<Resizer direction="horizontal" onResize={startHorizontalResize} />

			<div class="terminal-container" style="height: {bottomPanelHeight}%;">
				<Terminal />
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

	.dashboard-content {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.main-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.chart-container,
	.terminal-container {
		overflow: hidden;
	}
</style>

<script lang="ts">
	export let selectedSymbol = "EURUSD";
	export let selectedTimeframe = "M15";
	export let chartType = "candlesticks";
	export let isPlaying = false;
	export let isPaused = false;
	export let playbackSpeed = 1;

	const symbols = ["EURUSD", "GBPUSD", "USDJPY", "AUDUSD", "USDCAD"];
	const timeframes = ["M1", "M5", "M15", "M30", "H1", "H4", "D1", "W1"];

	function handlePlay() {
		isPlaying = true;
		isPaused = false;
		console.log("Play");
	}

	function handlePause() {
		isPaused = true;
		console.log("Pause");
	}

	function handleStop() {
		isPlaying = false;
		isPaused = false;
		console.log("Stop");
	}

	function handleStepForward() {
		console.log("Step Forward");
	}

	function handleStepBackward() {
		console.log("Step Backward");
	}
</script>

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
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
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
		<button
			class="toolbar-btn"
			title="Bar Chart"
			class:active={chartType === "bar"}
			onclick={() => (chartType = "bar")}
		>
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
		<button
			class="toolbar-btn"
			title="Candlesticks"
			class:active={chartType === "candlesticks"}
			onclick={() => (chartType = "candlesticks")}
		>
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
		<button
			class="toolbar-btn"
			title="Line Chart"
			class:active={chartType === "line"}
			onclick={() => (chartType = "line")}
		>
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

	<div class="toolbar-separator"></div>

	<!-- Playback Controls -->
	<div class="toolbar-section playback-controls">
		<button
			class="toolbar-btn"
			title="Step Backward"
			onclick={handleStepBackward}
			disabled={isPlaying && !isPaused}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<polygon points="11 19 2 12 11 5 11 19" />
				<polygon points="22 19 13 12 22 5 22 19" />
			</svg>
		</button>

		{#if !isPlaying || isPaused}
			<button class="toolbar-btn play-btn" title="Play" onclick={handlePlay}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="currentColor"
					stroke="none"
				>
					<polygon points="5 3 19 12 5 21 5 3" />
				</svg>
			</button>
		{:else}
			<button class="toolbar-btn pause-btn" title="Pause" onclick={handlePause}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="currentColor"
					stroke="none"
				>
					<rect x="6" y="4" width="4" height="16" />
					<rect x="14" y="4" width="4" height="16" />
				</svg>
			</button>
		{/if}

		<button
			class="toolbar-btn stop-btn"
			title="Stop"
			onclick={handleStop}
			disabled={!isPlaying}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="currentColor"
				stroke="none"
			>
				<rect x="5" y="5" width="14" height="14" rx="2" />
			</svg>
		</button>

		<button
			class="toolbar-btn"
			title="Step Forward"
			onclick={handleStepForward}
			disabled={isPlaying && !isPaused}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<polygon points="13 19 22 12 13 5 13 19" />
				<polygon points="2 19 11 12 2 5 2 19" />
			</svg>
		</button>

		<div class="speed-control">
			<label for="speed-slider" title="Playback Speed">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="12" cy="12" r="10" />
					<polyline points="12 6 12 12 16 14" />
				</svg>
			</label>
			<input
				id="speed-slider"
				type="range"
				min="0.25"
				max="4"
				step="0.25"
				bind:value={playbackSpeed}
				class="speed-slider"
				title="Playback Speed: {playbackSpeed}x"
			/>
			<span class="speed-label">{playbackSpeed}x</span>
		</div>
	</div>

	<div class="toolbar-separator"></div>

	<div class="toolbar-section">
		<select class="toolbar-select" bind:value={selectedSymbol}>
			{#each symbols as symbol}
				<option value={symbol}>{symbol}</option>
			{/each}
		</select>
		<select class="toolbar-select" bind:value={selectedTimeframe}>
			{#each timeframes as tf}
				<option value={tf}>{tf}</option>
			{/each}
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

<style>
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

	.playback-controls {
		gap: 4px;
		padding: 0 4px;
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

	.toolbar-btn:hover:not(:disabled) {
		background-color: var(--border-color);
	}

	.toolbar-btn.active {
		background-color: var(--accent);
		color: white;
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.toolbar-btn.play-btn {
		color: var(--accent);
	}

	.toolbar-btn.play-btn:hover:not(:disabled) {
		background-color: var(--accent);
		color: white;
	}

	.toolbar-btn.pause-btn {
		background-color: var(--accent);
		color: white;
	}

	.toolbar-btn.pause-btn:hover {
		background-color: var(--primary-hover);
	}

	.toolbar-btn.stop-btn:hover:not(:disabled) {
		background-color: #ef4444;
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

	.speed-control {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0 8px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 3px;
		height: 26px;
	}

	.speed-control label {
		display: flex;
		align-items: center;
		color: var(--text-secondary);
		cursor: pointer;
	}

	.speed-slider {
		width: 80px;
		height: 4px;
		background: var(--border-color);
		border-radius: 2px;
		outline: none;
		-webkit-appearance: none;
		appearance: none;
		cursor: pointer;
	}

	.speed-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 12px;
		height: 12px;
		background: var(--accent);
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.15s;
	}

	.speed-slider::-webkit-slider-thumb:hover {
		transform: scale(1.2);
	}

	.speed-slider::-moz-range-thumb {
		width: 12px;
		height: 12px;
		background: var(--accent);
		border-radius: 50%;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
	}

	.speed-slider::-moz-range-thumb:hover {
		transform: scale(1.2);
	}

	.speed-label {
		font-size: 11px;
		font-weight: 600;
		color: var(--text-primary);
		min-width: 32px;
		text-align: right;
		font-family: "Consolas", "Monaco", monospace;
	}
</style>

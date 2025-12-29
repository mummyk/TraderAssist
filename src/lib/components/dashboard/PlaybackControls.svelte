<script lang="ts">
	import ToolbarButton from "./ToolbarButton.svelte";

	export let isPlaying = false;
	export let isPaused = false;
	export let playbackSpeed = 1;

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

<div class="playback-controls">
	<ToolbarButton
		title="Step Backward"
		disabled={isPlaying && !isPaused}
		onclick={handleStepBackward}
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
	</ToolbarButton>

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

	<ToolbarButton
		title="Step Forward"
		disabled={isPlaying && !isPaused}
		onclick={handleStepForward}
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
	</ToolbarButton>

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

<style>
	.playback-controls {
		display: flex;
		align-items: center;
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

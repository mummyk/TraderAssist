<script lang="ts">
	export let direction: "vertical" | "horizontal" = "vertical";
	export let onResize: (e: MouseEvent) => void;
</script>

<div
	class="resizer {direction}"
	on:mousedown={onResize}
	role="separator"
	tabindex="0"
></div>

<style>
	.resizer {
		background-color: var(--border-color);
		flex-shrink: 0;
		transition: background-color 0.15s;
	}

	.resizer.vertical {
		width: 4px;
		cursor: ew-resize;
	}

	.resizer.horizontal {
		height: 4px;
		cursor: ns-resize;
	}

	.resizer:hover {
		background-color: var(--accent);
	}

	.resizer:active {
		background-color: var(--accent);
	}
</style>

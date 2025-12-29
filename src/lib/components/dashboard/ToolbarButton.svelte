<script lang="ts">
	export let title: string = "";
	export let active: boolean = false;
	export let disabled: boolean = false;
	export let onclick: (() => void) | undefined = undefined;
	export let children: import("svelte").Snippet;
</script>

<button class="toolbar-btn" class:active {title} {disabled} {onclick}>
	{@render children()}
</button>

<style>
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
</style>

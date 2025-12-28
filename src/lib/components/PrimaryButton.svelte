<script lang="ts">
	interface Props {
		disabled?: boolean;
		variant?: "primary" | "secondary";
		badge?: string;
		onclick?: (event: MouseEvent) => void;
		children?: import("svelte").Snippet;
	}

	let {
		disabled = false,
		variant = "primary",
		badge = undefined,
		onclick,
		children,
	}: Props = $props();

	function handleClick(event: MouseEvent) {
		if (!disabled && onclick) {
			onclick(event);
		}
	}
</script>

<button class="btn {variant}" class:disabled onclick={handleClick} {disabled}>
	{#if children}
		{@render children()}
	{/if}
	{#if badge}
		<span class="badge">{badge}</span>
	{/if}
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 500;
		border-radius: 4px;
		border: 1px solid transparent;
		cursor: pointer;
		transition: all 0.15s ease;
		font-family: inherit;
		white-space: nowrap;
	}

	/* Primary Style */
	.primary {
		background-color: var(--primary-color);
		color: #ffffff;
		border-color: var(--primary-color);
	}

	.primary:hover:not(.disabled) {
		background-color: var(--primary-hover);
		border-color: var(--primary-hover);
	}

	.primary:active:not(.disabled) {
		transform: translateY(1px);
	}

	/* Secondary Style */
	.secondary {
		background-color: var(--surface-color);
		color: var(--text-primary);
		border-color: var(--border-color);
	}

	.secondary:hover:not(.disabled) {
		background-color: var(--bg-secondary);
		border-color: var(--accent);
	}

	.secondary:active:not(.disabled) {
		transform: translateY(1px);
	}

	/* Disabled State */
	.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.badge {
		background-color: #ff6b6b;
		color: white;
		font-size: 10px;
		padding: 2px 6px;
		border-radius: 3px;
		text-transform: uppercase;
		font-weight: 600;
		letter-spacing: 0.3px;
	}
</style>

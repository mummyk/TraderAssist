<script lang="ts">
	export let onSelect: ((shape: string) => void) | undefined = undefined;

	const shapes = [
		{ name: "Trend Line", icon: "📈" },
		{ name: "Horizontal Line", icon: "➖" },
		{ name: "Vertical Line", icon: "|" },
		{ name: "Rectangle", icon: "▭" },
		{ name: "Circle", icon: "○" },
		{ name: "Arrow", icon: "→" },
		{ name: "Text", icon: "T" },
		{ name: "Fibonacci", icon: "🔢" },
	];

	function handleSelect(shape: string) {
		if (onSelect) {
			onSelect(shape);
		}
		console.log("Selected shape:", shape);
	}
</script>

<div class="shape-modal">
	<div class="shape-grid">
		{#each shapes as shape}
			<button
				class="shape-item"
				onclick={() => handleSelect(shape.name)}
				title={shape.name}
			>
				<span class="shape-icon">{shape.icon}</span>
				<span class="shape-name">{shape.name}</span>
			</button>
		{/each}
	</div>
</div>

<style>
	.shape-modal {
		min-width: 450px;
	}

	.shape-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 8px;
	}

	.shape-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 16px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.shape-item:hover {
		background-color: var(--border-color);
		border-color: var(--accent);
		transform: translateY(-2px);
	}

	.shape-icon {
		font-size: 24px;
	}

	.shape-name {
		font-size: 12px;
		font-weight: 500;
		color: var(--text-primary);
	}
</style>

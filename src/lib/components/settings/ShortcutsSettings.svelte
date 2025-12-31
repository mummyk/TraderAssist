<script lang="ts">
	import {
		shortcutsStore,
		type Shortcut,
	} from "../../../stores/shortcutsStore";

	let shortcuts = $state<Shortcut[]>([]);

	shortcutsStore.subscribe((state) => {
		shortcuts = state.shortcuts;
	});
</script>

<div class="settings-section">
	<h3 class="section-title">Keyboard Shortcuts</h3>
	<p class="section-description">View and customize keyboard shortcuts</p>

	<div class="shortcuts-list">
		{#each shortcuts as shortcut}
			<div class="shortcut-item">
				<div class="shortcut-info">
					<span class="shortcut-name">{shortcut.name}</span>
					<span class="shortcut-desc">{shortcut.description}</span>
				</div>
				<div class="shortcut-keys">
					{#each shortcut.keys as key}
						<kbd class="key"
							>{key
								.replace("cmd", "⌘")
								.replace("ctrl", "Ctrl")
								.replace("shift", "⇧")
								.replace("+", " + ")}</kbd
						>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.settings-section {
		max-width: 600px;
	}

	.section-title {
		font-size: 18px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 8px 0;
	}

	.section-description {
		font-size: 13px;
		color: var(--text-secondary);
		margin: 0 0 24px 0;
	}

	.shortcuts-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.shortcut-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 14px 16px;
		background-color: var(--bg-secondary);
		border-radius: 6px;
		transition: background-color 0.15s;
	}

	.shortcut-item:hover {
		background-color: var(--border-color);
	}

	.shortcut-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.shortcut-name {
		font-size: 13px;
		font-weight: 500;
		color: var(--text-primary);
	}

	.shortcut-desc {
		font-size: 12px;
		color: var(--text-secondary);
	}

	.shortcut-keys {
		display: flex;
		gap: 6px;
		align-items: center;
	}

	.key {
		display: inline-block;
		padding: 4px 8px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 4px;
		font-size: 11px;
		font-family: monospace;
		color: var(--text-primary);
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
	}
</style>

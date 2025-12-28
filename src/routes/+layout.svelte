<script>
	import "../app.css";
	import ThemeToggle from "../lib/components/ThemeToggle.svelte";
	import { page } from "$app/stores";

	$: isDashboard = $page.url.pathname.startsWith("/dashboard");
</script>

{#if isDashboard}
	<!-- Dashboard Layout with Titlebar -->
	<div class="app-window">
		<header class="titlebar no-select">
			<div class="titlebar-left">
				<svg
					class="app-icon"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M3 3v18h18" />
					<path d="M19 9l-5 5-4-4-3 3" />
				</svg>
				<span class="app-title">Trader Assist</span>
			</div>
			<div class="titlebar-right">
				<ThemeToggle />
			</div>
		</header>

		<div class="app-content">
			<slot />
		</div>

		<footer class="statusbar no-select">
			<div class="status-left">
				<span class="status-item">
					<span class="status-indicator ready"></span>
					Connected
				</span>
			</div>
			<div class="status-right">
				<span class="status-item">v1.0.0</span>
			</div>
		</footer>
	</div>
{:else}
	<!-- Welcome Page (no titlebar) -->
	<slot />
{/if}

<style>
	.app-window {
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow: hidden;
	}

	.titlebar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 12px;
		height: 40px;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		flex-shrink: 0;
	}

	.titlebar-left {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.app-icon {
		width: 18px;
		height: 18px;
		color: var(--accent);
	}

	.app-title {
		font-size: 13px;
		font-weight: 500;
		color: var(--text-primary);
	}

	.app-content {
		flex: 1;
		overflow: hidden;
	}

	.statusbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 12px;
		height: 24px;
		background-color: var(--bg-secondary);
		border-top: 1px solid var(--border-color);
		font-size: 11px;
		color: var(--text-secondary);
		flex-shrink: 0;
	}

	.status-left,
	.status-right {
		display: flex;
		gap: 16px;
	}

	.status-item {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.status-indicator {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.status-indicator.ready {
		background-color: #4caf50;
	}
</style>

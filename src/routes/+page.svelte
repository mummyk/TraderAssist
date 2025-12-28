<script lang="ts">
	import { goto } from "$app/navigation";
	import ThemeToggle from "../lib/components/ThemeToggle.svelte";
	import PrimaryButton from "../lib/components/PrimaryButton.svelte";

	function handleLoadApp() {
		goto("/dashboard");
	}

	function handleCloudConnect() {
		console.log("Cloud connection coming soon...");
	}
</script>

<svelte:head>
	<title>Trader Assist</title>
</svelte:head>

<div class="app-window">
	<!-- Title Bar -->
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

	<!-- Main Content -->
	<main class="content">
		<div class="welcome-section">
			<h1 class="welcome-title">Welcome to Trader Assist</h1>
			<p class="welcome-subtitle">
				AI-powered trading research, strategy building, and algorithmic trading
				platform
			</p>

			<div class="action-buttons">
				<PrimaryButton onclick={handleLoadApp}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
						<polyline points="9 22 9 12 15 12 15 22" />
					</svg>
					Open Terminal
				</PrimaryButton>
				<PrimaryButton variant="secondary" disabled badge="Soon">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" />
					</svg>
					Connect Cloud
				</PrimaryButton>
			</div>
		</div>

		<!-- Features Grid -->
		<div class="features-container">
			<h2 class="section-title">Core Features</h2>

			<div class="features-grid">
				<div class="feature-item">
					<div class="feature-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
							<path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
						</svg>
					</div>
					<div class="feature-content">
						<h3 class="feature-title">Trading Research</h3>
						<p class="feature-description">
							Real-time market data, analytics, and technical indicators
						</p>
					</div>
				</div>

				<div class="feature-item">
					<div class="feature-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<rect width="18" height="18" x="3" y="3" rx="2" />
							<path d="M3 9h18" />
							<path d="M9 21V9" />
						</svg>
					</div>
					<div class="feature-content">
						<h3 class="feature-title">Strategy Builder</h3>
						<p class="feature-description">
							Visual drag-and-drop builder for complex trading strategies
						</p>
					</div>
				</div>

				<div class="feature-item">
					<div class="feature-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<circle cx="12" cy="12" r="3" />
							<path
								d="M12 1v6m0 6v6m5.2-13.2l-4.2 4.2m0 6l4.2 4.2M1 12h6m6 0h6m-13.2 5.2l4.2-4.2m0-6l-4.2-4.2"
							/>
						</svg>
					</div>
					<div class="feature-content">
						<h3 class="feature-title">AI Algorithm Builder</h3>
						<p class="feature-description">
							Machine learning models for predictive trading bots
						</p>
					</div>
				</div>

				<div class="feature-item">
					<div class="feature-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path
								d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
							/>
							<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
							<line x1="12" x2="12" y1="22.08" y2="12" />
						</svg>
					</div>
					<div class="feature-content">
						<h3 class="feature-title">Backtesting Engine</h3>
						<p class="feature-description">
							Test strategies against historical data before deployment
						</p>
					</div>
				</div>
			</div>
		</div>
	</main>

	<!-- Status Bar -->
	<footer class="statusbar no-select">
		<div class="status-left">
			<span class="status-item">
				<span class="status-indicator ready"></span>
				Ready
			</span>
		</div>
		<div class="status-right">
			<span class="status-item">v1.0.0</span>
		</div>
	</footer>
</div>

<style>
	/* Same styles as before */
	.app-window {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background-color: var(--bg-primary);
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

	.content {
		flex: 1;
		overflow-y: auto;
		padding: 32px;
		background-color: var(--bg-primary);
	}

	.welcome-section {
		max-width: 600px;
		margin: 0 auto 48px;
		text-align: center;
	}

	.welcome-title {
		font-size: 28px;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 8px;
	}

	.welcome-subtitle {
		font-size: 14px;
		color: var(--text-secondary);
		margin-bottom: 24px;
		line-height: 1.5;
	}

	.action-buttons {
		display: flex;
		gap: 12px;
		justify-content: center;
	}

	.features-container {
		max-width: 900px;
		margin: 0 auto;
	}

	.section-title {
		font-size: 16px;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 16px;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 12px;
	}

	.feature-item {
		display: flex;
		gap: 12px;
		padding: 16px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		transition: all 0.15s ease;
		cursor: pointer;
	}

	.feature-item:hover {
		background-color: var(--bg-secondary);
		border-color: var(--accent);
	}

	.feature-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: var(--icon-bg);
		border-radius: 6px;
		flex-shrink: 0;
		color: var(--accent);
	}

	.feature-content {
		flex: 1;
	}

	.feature-title {
		font-size: 14px;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.feature-description {
		font-size: 12px;
		color: var(--text-secondary);
		line-height: 1.4;
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
		background-color: #22c55e;
	}

	.content::-webkit-scrollbar {
		width: 12px;
	}

	.content::-webkit-scrollbar-track {
		background: var(--bg-primary);
	}

	.content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 6px;
		border: 3px solid var(--bg-primary);
	}

	.content::-webkit-scrollbar-thumb:hover {
		background: var(--text-secondary);
	}

	@media (max-width: 768px) {
		.features-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

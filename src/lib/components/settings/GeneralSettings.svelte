<script lang="ts">
	import { settingsStore } from "../../../stores/settingsStore";
	import type { GeneralSettings } from "../../../stores/settingsStore";

	let settings = $state<GeneralSettings>({
		autoSave: true,
		showGrid: true,
		defaultChartType: "candlesticks",
	});

	settingsStore.subscribe((state) => {
		settings = state.general;
	});

	function handleToggle(key: keyof GeneralSettings) {
		settingsStore.updateGeneral(key, !settings[key] as any);
	}

	function handleSelectChange(key: keyof GeneralSettings, value: any) {
		settingsStore.updateGeneral(key, value);
	}
</script>

<div class="settings-section">
	<h3 class="section-title">General Settings</h3>
	<p class="section-description">Configure general application preferences</p>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="auto-save-toggle">Auto-save</label>
			<p class="setting-desc">Automatically save changes</p>
		</div>
		<label class="toggle">
			<input
				type="checkbox"
				id="auto-save-toggle"
				checked={settings.autoSave}
				onchange={() => handleToggle("autoSave")}
			/>
			<span class="toggle-slider"></span>
		</label>
	</div>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="show-grid-toggle">Show Grid</label>
			<p class="setting-desc">Display grid lines on charts</p>
		</div>
		<label class="toggle">
			<input
				type="checkbox"
				id="show-grid-toggle"
				checked={settings.showGrid}
				onchange={() => handleToggle("showGrid")}
			/>
			<span class="toggle-slider"></span>
		</label>
	</div>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="default-chart-type"
				>Default Chart Type</label
			>
			<p class="setting-desc">Default visualization type for new charts</p>
		</div>
		<select
			class="setting-select"
			id="default-chart-type"
			value={settings.defaultChartType}
			onchange={(e) =>
				handleSelectChange(
					"defaultChartType",
					(e.target as HTMLSelectElement).value
				)}
		>
			<option value="candlesticks">Candlesticks</option>
			<option value="bar">Bar Chart</option>
			<option value="line">Line Chart</option>
		</select>
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

	.setting-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 0;
		border-bottom: 1px solid var(--border-color);
	}

	.setting-item:last-child {
		border-bottom: none;
	}

	.setting-info {
		flex: 1;
	}

	.setting-label {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.setting-desc {
		font-size: 12px;
		color: var(--text-secondary);
		margin: 0;
	}

	.setting-select {
		padding: 8px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 13px;
		cursor: pointer;
		min-width: 150px;
	}

	.setting-select:focus {
		outline: none;
		border-color: var(--accent);
	}

	/* Toggle Switch */
	.toggle {
		position: relative;
		display: inline-block;
		width: 44px;
		height: 24px;
		cursor: pointer;
	}

	.toggle input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: var(--border-color);
		transition: 0.3s;
		border-radius: 24px;
	}

	.toggle-slider:before {
		position: absolute;
		content: "";
		height: 18px;
		width: 18px;
		left: 3px;
		bottom: 3px;
		background-color: white;
		transition: 0.3s;
		border-radius: 50%;
	}

	.toggle input:checked + .toggle-slider {
		background-color: var(--accent);
	}

	.toggle input:checked + .toggle-slider:before {
		transform: translateX(20px);
	}
</style>

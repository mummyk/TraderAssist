<script lang="ts">
	import { modalStore } from "../../stores/modalStore";
	import { fade } from "svelte/transition";

	let modalState = $state({
		isOpen: false,
		title: "",
		component: null as any,
		props: {},
	});

	$effect(() => {
		const unsubscribe = modalStore.subscribe((state) => {
			modalState = state;
		});

		return () => {
			unsubscribe();
		};
	});

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			modalStore.close();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Escape" && modalState.isOpen) {
			modalStore.close();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if modalState.isOpen}
	<div
		class="modal-backdrop"
		onclick={handleBackdropClick}
		transition:fade={{ duration: 200 }}
		role="presentation"
	>
		<div class="modal" transition:fade={{ duration: 200, delay: 100 }}>
			<div class="modal-header">
				<h3 class="modal-title">{modalState.title}</h3>
				<button
					class="close-btn"
					onclick={() => modalStore.close()}
					title="Close"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>
			<div class="modal-content">
				{#if modalState.component}
					{@const Component = modalState.component}
					<Component {...modalState.props} />
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		backdrop-filter: blur(2px);
	}

	.modal {
		background-color: var(--surface-color);
		border-radius: 8px;
		border: 1px solid var(--border-color);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
		min-width: 400px;
		max-width: 90vw;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--border-color);
		flex-shrink: 0;
	}

	.modal-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.close-btn:hover {
		background-color: var(--border-color);
		color: var(--text-primary);
	}

	.modal-content {
		padding: 20px;
		overflow-y: auto;
		flex: 1;
	}

	.modal-content::-webkit-scrollbar {
		width: 8px;
	}

	.modal-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.modal-content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>

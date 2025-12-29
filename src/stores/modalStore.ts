import { writable } from 'svelte/store';

export interface ModalState {
	isOpen: boolean;
	title: string;
	component: any;
	props: Record<string, any>;
}

function createModalStore() {
	const { subscribe, set } = writable<ModalState>({
		isOpen: false,
		title: '',
		component: null,
		props: {}
	});

	return {
		subscribe,
		open: (title: string, component: any, props: Record<string, any> = {}) => {
			set({
				isOpen: true,
				title,
				component,
				props
			});
		},
		close: () => {
			set({
				isOpen: false,
				title: '',
				component: null,
				props: {}
			});
		}
	};
}

export const modalStore = createModalStore();
export interface ToastOptions {
	id?: string;
	message: string;
	type: 'success' | 'error' | 'info';
	duration: number;
	position:
		| 'top'
		| 'bottom'
		| 'left'
		| 'right'
		| 'top-left'
		| 'top-right'
		| 'bottom-left'
		| 'bottom-right';
}

export class ToastManager {
	private toasts: ToastOptions[] = [];

	addToast(options: ToastOptions) {
		const id = Math.random().toString(36).substr(2, 9);
		this.toasts.push({ ...options, id });
		return id;
	}

	removeToast(id: string) {
		this.toasts = this.toasts.filter((toast) => toast.id !== id);
	}

	getToasts() {
		return this.toasts;
	}
}

export const toastManager = new ToastManager();

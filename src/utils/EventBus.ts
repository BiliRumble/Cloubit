import { Event, UnlistenFn } from '@tauri-apps/api/event';
import { Window } from '@tauri-apps/api/window';

type EventCallback<T = any> = (data: T) => void;

class EventBus {
	private events: Map<string, EventCallback[]> = new Map();
	private tauriListeners: Map<string, UnlistenFn> = new Map();
	private readonly windowId: string;

	constructor() {
		this.windowId = Window.getCurrent().label;
		this.setupTauriGlobalListener();
	}

	private async setupTauriGlobalListener() {
		this.events.forEach((_, eventName) => {
			this.registerTauriListener(eventName);
		});
	}

	private async registerTauriListener(eventName: string) {
		if (this.tauriListeners.has(eventName)) return;

		const { listen } = await import('@tauri-apps/api/event');
		const unlisten = await listen(eventName, (event: Event<any>) => {
			// 过滤来自当前窗口的事件
			if (event.payload?._sourceWindow === this.windowId) return;
			this.emit(event.event, event.payload?.data, false);
		});

		this.tauriListeners.set(eventName, unlisten);
	}

	on<T>(eventName: string, callback: EventCallback<T>) {
		const handlers = this.events.get(eventName) || [];
		handlers.push(callback);
		this.events.set(eventName, handlers);

		// 动态注册Tauri监听（确保唯一性）
		this.registerTauriListener(eventName);
	}

	off(eventName: string, callback: EventCallback) {
		const handlers = this.events.get(eventName);
		if (handlers) {
			const newHandlers = handlers.filter((fn) => fn !== callback);
			this.events.set(eventName, newHandlers);

			// 无回调时移除Tauri监听
			if (newHandlers.length === 0) {
				this.unregisterTauriListener(eventName);
			}
		}
	}

	private unregisterTauriListener(eventName: string) {
		const unlisten = this.tauriListeners.get(eventName);
		if (unlisten) {
			unlisten();
			this.tauriListeners.delete(eventName);
		}
	}

	async emit<T>(eventName: string, data?: T, global: boolean = true) {
		console.debug('EventBus.emit', eventName, data, global);

		// 本地触发
		const handlers = this.events.get(eventName);
		if (handlers) {
			handlers.forEach((callback) => {
				try {
					callback(data);
				} catch (e) {
					console.error(`处理事件 ${eventName} 出错:`, e);
				}
			});
		}

		// 全局触发时添加来源标识
		if (global) {
			const { emit } = await import('@tauri-apps/api/event');
			await emit(eventName, {
				data: data,
				_sourceWindow: this.windowId, // 注入窗口标识
			});
		}
	}

	once<T>(eventName: string, callback: EventCallback<T>) {
		const wrapper = (data: T) => {
			callback(data);
			this.off(eventName, wrapper);
		};
		this.on(eventName, wrapper);
	}
}

export const eventBus = new EventBus();

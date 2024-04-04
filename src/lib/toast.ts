import type { ToastSettings } from '@skeletonlabs/skeleton';
import { appConfiguration } from './stores/configuration';

export type CustomToastSettings = ToastSettings & { title?: string | undefined };

export function showToast(settings: CustomToastSettings): ToastSettings {
	if (settings.autohide !== false) {
		settings = {
			autohide: true,
			timeout: appConfiguration.getToastTime(),
			...settings
		};
	}
	if (settings.title !== undefined) {
		const { title, message } = settings;
		settings.message = `<strong>${title}</strong><br>${message}`;
	}
	return settings;
}

export function showSuccessToast(settings: CustomToastSettings): ToastSettings {
	return showToast({
		...settings,
		background: 'variant-filled-success'
	});
}

export function showWarningToast(settings: CustomToastSettings): ToastSettings {
	return showToast({
		...settings,
		background: 'variant-filled-warning'
	});
}

export function showErrorToast(settings: CustomToastSettings): ToastSettings {
	return showToast({
		...settings,
		background: 'variant-filled-error'
	});
}

import { os } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/shell';
import { PATHS } from '$lib/constants';

export async function getOperatingSystem() {
	try {
		const platform = await os.platform();
		let osName = '';

		switch (platform) {
			case 'darwin':
				osName = 'macOS';
				break;
			case 'linux':
				osName = 'Linux';
				break;
			case 'win32':
				osName = 'Windows';
				break;
			default:
				osName = 'Unsopported';
		}

		console.log(`The operating system is: ${osName}`);
		return osName as 'macOS' | 'Linux' | 'Windows' | 'Unsupported';
	} catch (error) {
		console.error('Error determining the operating system:', error);
	}
}

export async function launchLV1() {
	try {
		await open(PATHS.LV1_APPLICATION);
	} catch (error) {
		console.error('Failed to open folder:', error);
	}
}

export async function launchWavesCentral() {
	try {
		await open(PATHS.WAVES_CENTRAL);
	} catch (error) {
		console.error('Failed to open folder:', error);
	}
}

export function shutdownComputer() {
	confirm('Are you sure you want to shut down the system?') &&
		invoke('shutdown_system').catch((error) => {
			console.error('Failed to shut down the system:', error);
		});
}

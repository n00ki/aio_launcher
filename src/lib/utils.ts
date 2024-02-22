import { os } from '@tauri-apps/api';

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

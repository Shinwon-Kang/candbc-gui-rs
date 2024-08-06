import { g_dbcs, g_selected_dbc, g_dbc_info } from '../lib/stores.js';

import { invoke } from '@tauri-apps/api/tauri';
import { info, error } from 'tauri-plugin-log-api';

export async function select_dbc(dbc) {
	g_selected_dbc.set(dbc);

	await invoke('get_dbc_info', { dbc })
		.then((info) => {
			let dbc_info = JSON.parse(info);
			g_dbc_info.set({
				file_name: dbc_info.file_name,
				version: dbc_info.version,
				symbols: dbc_info.symbols,
				nodes: dbc_info.nodes,
				messages: dbc_info.messages,
				comments: dbc_info.comments
			});
		})
		.catch((err) => {
			error('get dbc info failed');
		});

	// todo: get messages info
}

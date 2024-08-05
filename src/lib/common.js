import { g_dbcs, g_selected_dbc, g_summary_info } from '../lib/stores.js';

async function selecte_dbc(dbc) {
    g_selected_dbc.set(dbc);
    g_summary_info.set({
        file_name: 'sample',
        version: 'sample',
        symbols: [
            'sample'
        ],
        nodes: [
            'sample'
        ],
        messages_info: [
            {
                message_name: 'sample',
                message_cnt: 9999
            }
        ],
        comments: [
            ['sample']
        ]
    });
}
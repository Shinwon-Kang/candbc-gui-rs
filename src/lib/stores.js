import { writable } from 'svelte/store';

export const g_dbcs = writable([]);
export const g_selected_dbc = writable();
export const g_summary_info = writable(
    {
        file_name: 'file_name',
        version: 'version',
        symbols: ['symbol1', 'symbol2', 'symbol3', 'symbol4'],
        nodes: ['node1', 'node2', 'node3'],
        messages_info: [
            {
                message_name: 'message1',
                message_cnt: 9999
            },
            {
                message_name: 'message2',
                message_cnt: 9999
            },
            {
                message_name: 'message3',
                message_cnt: 9999
            }
        ],
        comments: [
            ['comments1'],
            ['comments2', 'comments3', 'comments4'],
            [
                'comments5',
                'comments6',
                'comments7'
            ],
            ['comments8', 'comments9', 'comments10'],
            ['comments11', 'comments12', 'comments13']
        ]
    }
);
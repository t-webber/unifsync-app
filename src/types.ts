declare global {
    interface Window {
        __TAURI__: {
            core: {
                invoke: (cmd: string, props?: object) => any;
            };
        };
    }
}

export interface NoteProps {
    readonly id: number;
    readonly title: string;
    readonly content: string;
}

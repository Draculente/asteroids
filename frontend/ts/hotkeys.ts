export interface Hotkey {
    key: string;
    description: string;
    predicate: HotkeyPredicate;
    callback: (event: KeyboardEvent) => void;
}

export type HotkeyPredicate = (event: KeyboardEvent) => boolean;


export function key(key: string): HotkeyPredicate {
    return (event) => event.key === key;
}

export default class HotkeyHandler {
    hotkeyStore: Hotkey[] = [];

    constructor() {
    }

    handleHotkey(e: KeyboardEvent) {
        this.hotkeyStore.forEach((hotkey) => {
            if (hotkey.predicate(e)) {
                hotkey.callback(e);
            }
        })
    }

    registerHotkeys(hotkeys: Hotkey[]) {
        this.hotkeyStore = this.hotkeyStore.concat(hotkeys);
    }

    get hotkeys(): Hotkey[] {
        return this.hotkeyStore;
    }
}
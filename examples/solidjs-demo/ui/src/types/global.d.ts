export { };

type DataTypes = string | number | boolean | Uint8Array;

declare global {
    interface Webui {
        call(fn: string, ...args: DataTypes[]): Promise<string>;
    }
    var webui: Webui;
}
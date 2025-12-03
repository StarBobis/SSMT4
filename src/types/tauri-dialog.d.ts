declare module '@tauri-apps/api/dialog' {
  export interface OpenOptions {
    directory?: boolean;
    multiple?: boolean;
    filters?: Array<{ name: string; extensions: string[] }>;
    defaultPath?: string;
    title?: string;
  }

  export function open(options?: OpenOptions): Promise<string | string[] | null>;
}

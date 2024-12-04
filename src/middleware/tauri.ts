import { invoke } from '@tauri-apps/api/core';

export const Tauri = {
  load_key_heatmap,
};

export type Heatmap = {
  [key: string]: number;
};

async function load_key_heatmap(): Promise<Heatmap> {
  return invoke('load_key_heatmap');
}

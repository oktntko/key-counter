import { invoke } from '@tauri-apps/api/core';

export const Tauri = {
  start_listen,
  load_key_heatmap,
};

export type Heatmap = {
  [key: string]: number;
};

async function start_listen(): Promise<void> {
  return invoke('start_listen');
}

async function load_key_heatmap(): Promise<Heatmap> {
  return invoke('load_key_heatmap');
}

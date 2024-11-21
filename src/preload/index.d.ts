import { ElectronAPI } from '@electron-toolkit/preload'

declare global {
  interface Window {
    electron: ElectronAPI
    api: unknown
    electronAPI: {
      onUpdateCounter: (callback: (key: string) => void) => void
    }
  }
}

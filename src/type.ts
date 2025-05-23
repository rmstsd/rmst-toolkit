import 'react'

export type SettingData = {
  editorPaths: string[]
  cmdPath: string
  projectPaths: string[]
  notes: string[]
  historyOpenedUrls: string[]
}

export type AppBaseInfo = {
  appPath: string
  appVersion: string
  appName: string
  node: string
  chrome: string
  electron: string
}

declare module 'react' {
  interface HTMLAttributes<T> extends AriaAttributes, DOMAttributes<T> {
    'data-tauri-drag-region'?: boolean
  }
}

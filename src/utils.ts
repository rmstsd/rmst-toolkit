import { getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useEffect } from 'react'

type Cb = (c: { appWindow: WebviewWindow; focused: boolean }) => void

export const useFocus = (cb: Cb) => {
  useEffect(() => {
    const appWindow = getCurrentWebviewWindow()

    const un = appWindow.onFocusChanged(({ payload: focused }) => {
      cb({ appWindow, focused })
    })

    return () => {
      un.then(un => un())
    }
  }, [])
}

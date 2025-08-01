import { Button } from '@arco-design/web-react'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useRef, useState } from 'react'
import ResizeObserver from 'rc-resize-observer'
import { SettingData } from '../type'
import clsx from 'clsx'
import { useFocus } from '../utils'

export default function QuickInput() {
  const [notes, setNotes] = useState([])

  const sizeRef = useRef({ width: 0, height: 0 })

  useEffect(() => {
    invoke('getSetting').then((data: SettingData) => {
      setNotes(data.notes)
    })
  }, [])

  useFocus(({ appWindow, focused }) => {
    updateSize()
  })

  const updateSize = () => {
    invoke('setWindowSize', { width: sizeRef.current.width, height: sizeRef.current.height })
  }

  return (
    <ResizeObserver
      onResize={size => {
        sizeRef.current = { width: size.width, height: size.height }
        updateSize()
      }}
    >
      <div className="quick-input p-[6px] w-[200px]">
        <div data-tauri-drag-region className="h-[22px] bg-orange-400 flex mb-[5px]">
          <Button size="mini" className={clsx('win-not-drag h-full')} onClick={() => invoke('hideWindow')}>
            x
          </Button>
        </div>

        <div className="flex flex-col gap-[6px]">
          {notes.map((item, index) => (
            <Button
              size="small"
              key={index}
              type="default"
              className="!border-gray-300 !text-gray-800"
              onClick={() => invoke('CopyAndPaste', { content: item })}
            >
              {item}
            </Button>
          ))}
        </div>
      </div>
    </ResizeObserver>
  )
}

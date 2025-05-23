import { Button } from '@arco-design/web-react'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import ResizeObserver from 'rc-resize-observer'
import { SettingData } from '../type'
import clsx from 'clsx'

export default function QuickInput() {
  const [notes, setNotes] = useState([])

  useEffect(() => {
    invoke('getSetting').then((data: SettingData) => {
      setNotes(data.notes)
    })
  }, [])

  return (
    <ResizeObserver
      onResize={size => invoke('updateQuickInputWindowSize', { size: { width: size.width, height: size.height } })}
    >
      <div className="quick-input p-[6px] w-[200px]">
        <div data-tauri-drag-region className="h-[22px] bg-orange-400 flex mb-[5px]">
          <Button size="mini" className={clsx('win-not-drag h-full')} onClick={() => invoke('hideQuickInputWindow')}>
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

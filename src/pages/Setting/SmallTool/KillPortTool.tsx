import { Button, Form, InputNumber, Message } from '@arco-design/web-react'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useRef, useState } from 'react'

export default function KillPortTool() {
  const ref = useRef(null)

  useEffect(() => {
    ref.current?.focus()

    document.onvisibilitychange = () => {
      if (document.visibilityState === 'visible') {
        ref.current?.focus()
      }
    }
  }, [])

  const [form] = Form.useForm()

  const [loading, setLoading] = useState(false)

  const kill = async () => {
    await form.validate()

    setLoading(true)

    invoke('killPort', { port: form.getFieldValue('port') })
      .then(res => {
        console.log('res', res)
        Message.success({ content: '成功', position: 'bottom' })
      })
      .catch(err => {
        Message.error({ content: err, position: 'bottom' })
      })
      .finally(() => {
        setLoading(false)
      })
  }

  return (
    <Form form={form}>
      <Form.Item label="端口号" field="port" rules={[{ required: true }]}>
        <InputNumber ref={ref} placeholder="端口号" />
      </Form.Item>

      <Form.Item label=" ">
        <Button loading={loading} onClick={kill}>
          kill
        </Button>
      </Form.Item>
    </Form>
  )
}

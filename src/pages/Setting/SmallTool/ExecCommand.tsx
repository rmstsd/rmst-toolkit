import { Button, Divider, Form, Input, Message } from '@arco-design/web-react'
import { IconDelete } from '@arco-design/web-react/icon'
import { invoke } from '@tauri-apps/api/core'
import { Fragment, useEffect, useState } from 'react'
interface CmdItem {
  label: string
  cmd: string
  arg: string
  currentDir: string
}

interface FormValues {
  commands: CmdItem[]
}

export default function ExecCommand() {
  const [form] = Form.useForm<FormValues>()
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    init()
  }, [])

  const init = () => {
    invoke('getCommands').then((commands: CmdItem[]) => {
      form.setFieldsValue({ commands })
    })
  }

  const saveHandler = () => {
    form.validate().then(values => {
      console.log(values)

      invoke('saveCommands', { commands: values.commands }).then(() => {
        Message.success('保存成功')

        init()
      })
    })
  }

  const execCommand = (index: number) => {
    form.validate().then(values => {
      setLoading(true)
      const cmd = values.commands[index]

      invoke('execCommand', { label: cmd.label })
        .then(() => {
          Message.success('执行成功')
        })
        .catch(error => {
          Message.error(`执行失败: ${error}`)
        })
        .finally(() => {
          setLoading(false)
        })
    })
  }

  return (
    <Form
      form={form}
      labelAlign="right"
      initialValues={{
        commands: [
          { label: 's', cmd: 'd', arg: 'f', currentDir: 'g' },
          { label: '2', cmd: 'd3', arg: 'f4', currentDir: '4g' }
        ]
      }}
    >
      <Form.Item label=" ">
        <Button onClick={() => saveHandler()}>保存</Button>
      </Form.Item>

      <Form.Item label="命令列表" rules={[{ required: true }]}>
        <Form.List field="commands">
          {(fields, { add, remove }) => {
            console.log(fields)
            return (
              <>
                {fields.map((item, index) => {
                  return (
                    <Fragment key={item.key}>
                      <div className="flex gap-[10px]">
                        <div className="flex-grow grid" style={{ gridTemplateColumns: 'repeat(2, 1fr)', columnGap: 8 }}>
                          <Form.Item label="名称" field={item.field + '.label'} rules={[{ required: true }]}>
                            <Input placeholder="" />
                          </Form.Item>
                          <Form.Item label="命令" field={item.field + '.cmd'} rules={[{ required: true }]}>
                            <Input placeholder="node" />
                          </Form.Item>
                          <Form.Item label="参数" field={item.field + '.arg'} rules={[{ required: true }]}>
                            <Input placeholder="index.js" />
                          </Form.Item>
                          <Form.Item label="工作目录" field={item.field + '.currentDir'} rules={[{ required: true }]}>
                            <Input placeholder="E:/rmst-sd" />
                          </Form.Item>
                        </div>

                        <div className="flex gap-2">
                          <Button loading={loading} onClick={() => execCommand(index)}>
                            执行
                          </Button>

                          <Button
                            icon={<IconDelete />}
                            shape="circle"
                            status="danger"
                            className="shrink-0"
                            onClick={() => remove(index)}
                          />
                        </div>
                      </div>
                      <Divider type="horizontal" style={{ margin: '8px 0', marginTop: 0 }} />
                    </Fragment>
                  )
                })}

                <div className="flex gap-1">
                  <Button onClick={() => add()}>add</Button>
                </div>
              </>
            )
          }}
        </Form.List>
      </Form.Item>
    </Form>
  )
}

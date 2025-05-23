import OpenFolder from './pages/OpenFolder'
import Setting from './pages/Setting'
import QuickInput from './pages/QuickInput'

function App() {
  const hash = location.hash.slice(1)

  return (
    <>
      {hash === 'openFolder' && <OpenFolder />}
      {hash === 'setting' && <Setting />}
      {hash === 'quickInput' && <QuickInput />}
    </>
  )
}

export default App

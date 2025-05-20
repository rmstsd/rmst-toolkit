import { execSync } from 'child_process'

// cargo tauri build --debug
execSync('pnpm tauri build --debug', {
  stdio: 'inherit',
  env: {
    ...process.env,
    TAURI_SIGNING_PRIVATE_KEY:
      'dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5SCtsVVZzZVpzcUtHZWJqOEh6Uy9BclZBM0VhWDd1QWVlMVY0Sjk4TmtFWUFBQkFBQUFBQUFBQUFBQUlBQUFBQXdoOElHYkI5UXJ1YjF3bEZ6aVNBTUovcmRrOStJSEdpd24zQkgza3dXejU2NHdLTHczT2VGZ2JVVURHQjJocE5SMkNMYlFnV1JqUVZhMGw3UElPeERTd2M0b21XclIvSWR0WHJBUUlPT2N6cHpOcmtRYnBFM2VpYWowTUxXak9xcXJveEl2bDQzdGM9Cg==',
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: 'rmst.4480'
  }
})

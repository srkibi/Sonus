import { app, BrowserWindow } from 'electron'

function createWindow() {
  const win = new BrowserWindow({
    width: 1720,
    height: 1080,
    webPreferences: {
      nodeIntegration: true
    }
  })
  win.loadFile('src/interface/window.html')
}

app.whenReady().then(createWindow)
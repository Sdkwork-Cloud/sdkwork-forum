import React from 'react'
import { Routes, Route } from 'react-router-dom'
import { AppShell } from './shell/AppShell'
import { routes } from './routes/routes'

function App() {
  return (
    <AppShell>
      <Routes>
        {routes.map((route) => (
          <Route
            key={route.path}
            path={route.path}
            element={route.element}
          />
        ))}
      </Routes>
    </AppShell>
  )
}

export default App

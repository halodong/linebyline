import CircularProgress from '@mui/material/CircularProgress'
import { HoxRoot } from 'hox'
import { StrictMode, Suspense } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App'
import { FallBackContainer } from './components/FallBack'
import { enableMapSet } from 'immer'
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import __MF__ from '@/context'
import './normalize.css'

enableMapSet()

const Main = () => {
  return (
    <Suspense
      fallback={
        <FallBackContainer>
          <CircularProgress />
        </FallBackContainer>
      }
    >
      <App />
    </Suspense>
  )
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <HoxRoot>
      <BrowserRouter>
        <Main />
      </BrowserRouter>
    </HoxRoot>
  </StrictMode>,
)

import './App.css'
import { get_utc_info } from './functions/date_utc'

function App() {

  return (
    <>
      <div>
        <h1>Hello from NGINX !</h1>
        <p>{get_utc_info()}</p>
      </div>
    </>
  )
}

export default App

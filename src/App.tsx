import { useEffect } from 'react'
import GlobalHeader from './components/GlobalHeader'
import WorkspaceCanvas from './components/Canvas/WorkspaceCanvas'
import Footer from './components/Footer'

function App() {
    useEffect(() => {
        console.log('Kinetic Terminal initialized')
    }, [])

    return (
        <div className="app-container">
            <GlobalHeader />
            <WorkspaceCanvas />
            <Footer />
        </div>
    )
}

export default App

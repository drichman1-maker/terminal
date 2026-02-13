import './GlobalHeader.css'

export default function GlobalHeader() {
    return (
        <header className="global-header">
            <div className="ticker-tape">
                <span className="ticker-item">[&gt;] BTC USD PERP</span>
                <span className="ticker-divider">|</span>
                <span className="ticker-item">VOL 4.2B</span>
                <span className="ticker-divider">|</span>
                <span className="ticker-item">OI 12.1B</span>
                <span className="ticker-divider">|</span>
                <span className="ticker-item">FR 0.01%</span>
                <span className="ticker-divider">|</span>
                <span className="ticker-item">{new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true })} EST</span>
            </div>
            <div className="header-actions">
                <button className="header-btn">[-]</button>
                <button className="header-btn">[X]</button>
            </div>
        </header>
    )
}

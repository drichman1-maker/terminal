import './Footer.css'

export default function Footer() {
    return (
        <footer className="footer">
            <div className="connection-status">
                <span className="status-item">
                    <span className="status-dot connected"></span>
                    Binance
                </span>
                <span className="status-item">
                    <span className="status-dot connected"></span>
                    OKX
                </span>
                <span className="status-item">
                    <span className="status-dot disconnected"></span>
                    Bybit
                </span>
                <span className="status-item">
                    <span className="status-dot connected"></span>
                    Coinbase
                </span>
            </div>
            <div className="latency-monitor">
                <span className="latency-item">Latency: <span className="data">12ms</span></span>
            </div>
        </footer>
    )
}

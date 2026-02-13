import './WorkspaceCanvas.css'

export default function WorkspaceCanvas() {
    return (
        <main className="workspace-canvas">
            <div className="workspace-grid">
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>1. TICKER LIST</h3>
                        <p>BTC +2.1%</p>
                        <p>ETH -1.2%</p>
                        <p>SOL +4.5%</p>
                    </div>
                </div>
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>2. MAIN CHART / ORDERBOOK</h3>
                        <p>Candlestick view placeholder</p>
                    </div>
                </div>
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>4. NEWS FEED</h3>
                        <p>- FED RATES...</p>
                        <p>- BINANCE L...</p>
                    </div>
                </div>
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>3. DEPTH CHART</h3>
                        <p>[||||||||||   ]</p>
                    </div>
                </div>
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>5. POSITIONS / RISK</h3>
                        <p>ASSET | LEV | PNL | LIQ PRICE</p>
                    </div>
                </div>
                <div className="grid-cell">
                    <div className="module-placeholder">
                        <h3>6. COMMAND BAR</h3>
                        <p>&gt; SOL OI HEAT</p>
                    </div>
                </div>
            </div>
        </main>
    )
}

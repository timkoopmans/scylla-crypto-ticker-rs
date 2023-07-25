const chartHeight = Math.floor(document.documentElement.clientHeight / 2);
const resolution = 2000;

const chart = LightweightCharts.createChart(document.getElementById('chart'), {
    width: document.documentElement.clientWidth,
    height: chartHeight,
    timeScale: {
        visible: true,
        timeVisible: true,
        secondsVisible: true,
    },
    layout: {
        background: { color: '#222' },
        textColor: '#DDD',
    },
    grid: {
        vertLines: { color: '#444' },
        horzLines: { color: '#444' },
    },
});

// Setting the border color for the vertical axis
chart.priceScale().applyOptions({
    borderColor: "#71649C",
});

// Setting the border color for the horizontal axis
chart.timeScale().applyOptions({
    borderColor: "#71649C",
});

const candlestickSeries = chart.addCandlestickSeries();

const symbol = 'ETH'
setupAutoRefresh();

function updateData() {
    fetch(`http://localhost:8000/data/${symbol}/60s`)
        .then(response => response.json())
        .then(data => {
            const formattedData = data.map(datum => ({
                time: datum.time_bucket * resolution / 1000 / 2,
                open: datum.open_price,
                high: datum.high_price,
                low: datum.low_price,
                close: datum.close_price,
            }));
            candlestickSeries.setData(formattedData);
            chart.timeScale().fitContent();
        });

    updateLiveTrades('ETH');
}

function setupAutoRefresh() {
    // Call updateChartData initially
    updateData();

    // Set up the auto-refresh interval
    setInterval(updateData, resolution);
}

// Function to fetch and append live trades to the container
function updateLiveTrades(symbol) {
    fetch(`http://localhost:8000/trades/${symbol}`)
        .then(response => response.json())
        .then(data => {
            // Get the container element
            const tradeContainer = document.getElementById('tradeContainer');

            // Clear the existing content in the container
            tradeContainer.innerHTML = '';

            // Create a table to hold the trade data
            const tradeTable = document.createElement('table');
            tradeTable.classList.add('trade-table');

            // Add table headers
            const headerRow = tradeTable.insertRow();
            const headers = ['ID', 'Time', 'Exchange', 'Base', 'Quote', 'Price', 'Quantity'];
            headers.forEach(headerText => {
                const headerCell = document.createElement('th');
                headerCell.textContent = headerText;
                headerRow.appendChild(headerCell);
            });

            // Append the new trade data to the table
            for (let i = data.length - 1; i >= 0; i--) {
                const datum = data[i];
                const row = tradeTable.insertRow();

                const timestamp = new Date(datum.timestamp).toLocaleString();

                const cellId = row.insertCell();
                cellId.textContent = datum.id;

                const cellTimestamp = row.insertCell();
                cellTimestamp.textContent = timestamp;

                const cellExchange = row.insertCell();
                cellExchange.textContent = datum.exchange;

                const cellBase = row.insertCell();
                cellBase.textContent = datum.base;

                const cellQuote = row.insertCell();
                cellQuote.textContent = datum.quote;

                const cellPrice = row.insertCell();
                cellPrice.textContent = datum.price;

                const cellQty = row.insertCell();
                cellQty.textContent = datum.qty;
            }

            // Append the table to the container
            tradeContainer.appendChild(tradeTable);
        })
        .catch(error => {
            console.error('Error fetching live trades:', error);
        });
}


const chartHeight = Math.floor(document.documentElement.clientHeight / 2);
const resolution = 2000;
const symbol = 'ETH';
const chartOptions = {
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
};

const chart = createChart('chart', chartOptions);
const candlestickSeries = chart.addCandlestickSeries();
setupAutoRefresh();

function createChart(id, options) {
    const chart = LightweightCharts.createChart(document.getElementById(id), options);
    setAxisColor(chart, '#71649C');
    return chart;
}

function setAxisColor(chart, color) {
    chart.priceScale().applyOptions({ borderColor: color });
    chart.timeScale().applyOptions({ borderColor: color });
}

function fetchData(url) {
    return fetch(url).then(response => response.json());
}

function updateData() {
    fetchData(`http://localhost:8000/data/${symbol}/60s`)
        .then(formatData)
        .then(data => {
            candlestickSeries.setData(data);
            chart.timeScale().fitContent();
        })
        .catch(console.error);

    updateLiveTrades(symbol);
}

function formatData(data) {
    return data.map(datum => ({
        time: datum.time_bucket * resolution / 1000 / 2,
        open: datum.open_price,
        high: datum.high_price,
        low: datum.low_price,
        close: datum.close_price,
    }));
}

function setupAutoRefresh() {
    updateData();
    setInterval(updateData, resolution);
}

function updateLiveTrades(symbol) {
    fetchData(`http://localhost:8000/trades/${symbol}`)
        .then(createTradeTable)
        .then(table => {
            const tradeContainer = document.getElementById('tradeContainer');
            tradeContainer.innerHTML = '';
            tradeContainer.appendChild(table);
        })
        .catch(console.error);
}

function createTradeTable(data) {
    const tradeTable = document.createElement('table');
    tradeTable.classList.add('trade-table');
    addHeaders(tradeTable, ['ID', 'Time', 'Exchange', 'Base', 'Quote', 'Price', 'Quantity']);
    addRows(tradeTable, data);
    return tradeTable;
}

function addHeaders(table, headers) {
    const headerRow = table.insertRow();
    headers.forEach(headerText => {
        const headerCell = document.createElement('th');
        headerCell.textContent = headerText;
        headerRow.appendChild(headerCell);
    });
}

function addRows(table, data) {
    for (let i = data.length - 1; i >= 0; i--) {
        const datum = data[i];
        const row = table.insertRow();
        const cells = [
            datum.id,
            new Date(datum.timestamp).toLocaleString(),
            datum.exchange,
            datum.base,
            datum.quote,
            datum.price,
            datum.qty,
        ];
        cells.forEach(cellText => {
            const cell = row.insertCell();
            cell.textContent = cellText;
        });
    }
}

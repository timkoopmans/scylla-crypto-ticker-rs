const chartHeight = Math.floor(document.documentElement.clientHeight / 2);
const resolution = 2000;

const chart = LightweightCharts.createChart(document.getElementById('chart'), {
    width: document.documentElement.clientWidth,
    height: chartHeight,
    timeScale: {
        visible: false,  // Enable visibleRange
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

// Get the symbol from the URL
let symbol = window.location.pathname.split('/');
symbol = symbol[symbol.length - (symbol[symbol.length - 1] ? 1 : 2)];
symbol = 'ETH'
setupAutoRefresh();

// Start with data from last 5 minutes
fetch(`http://localhost:8000/data/${symbol}/5m`)
    .then(response => response.json())
    .then(data => {
        const formattedData = data.map(datum => ({
            time: datum.time_bucket * resolution / 1000,
            open: datum.open_price,
            high: datum.high_price,
            low: datum.low_price,
            close: datum.close_price,
        }));
        candlestickSeries.setData(formattedData);
        chart.timeScale().fitContent();
    })
    .then(() => {
        // Subscribe to the visibleTimeRangeChange event after the chart has been initialized and data is set
        chart.timeScale().subscribeVisibleTimeRangeChange((visibleRange) => {
            if (!visibleRange) {
                return;
            }

            const visibleDurationInSeconds = visibleRange.to - visibleRange.from;

            if (visibleDurationInSeconds < 900) {
                const startString = new Date((visibleRange.from - 2 * 7 * 24 * 60 * 60) * 1000).toISOString();
                const endString = new Date((visibleRange.to + 2 * 7 * 24 * 60 * 60) * 1000).toISOString();
                fetch(`http://localhost:8000/data/${symbol}/${startString}/${endString}`)
                    .then(response => response.json())
                    .then(data => {
                        const formattedData = data.map(datum => ({
                            time: datum.time_bucket * resolution / 1000,
                            open: datum.open_price,
                            high: datum.high_price,
                            low: datum.low_price,
                            close: datum.close_price,
                        }));
                        candlestickSeries.setData(formattedData);
                    });
                chart.timeScale().fitContent();
            }
        });
    });

// Fetch data for this symbol
fetch(`http://localhost:8000/data/${symbol}`)
    .then(response => response.json())
    .then(data => {
        const formattedData = data.map(datum => ({
            time: datum.time_bucket * resolution / 1000,
            open: datum.open_price,
            high: datum.high_price,
            low: datum.low_price,
            close: datum.close_price,
        }));
        candlestickSeries.setData(formattedData);
    });

document.addEventListener('DOMContentLoaded', (event) => {
    // Your code here...
    document.getElementById('goto-button').addEventListener('click', () => {
        const dateInput = document.getElementById('goto-date').value;
        const date = new Date(dateInput);
        const timestamp = date.getTime() / 1000; // Convert to seconds

        chart.timeScale().fitContent();

        // Convert the input to a Date object
        const startDate = new Date(dateInput);

        // Calculate the end date by adding 4 * 7 days to the start date
        const endDate = new Date(startDate);
        endDate.setDate(startDate.getDate() + 4 * 7);

        // Format the dates in RFC3339 format
        const startString = startDate.toISOString();
        const endString = endDate.toISOString();

        // Fetch data for the week of the selected date
        fetch(`http://localhost:8000/data/${symbol}/${startString}/${endString}`)
            .then(response => response.json())
            .then(data => {
                const formattedData = data.map(datum => ({
                    time: datum.time_bucket * resolution / 1000,
                    open: datum.open_price,
                    high: datum.high_price,
                    low: datum.low_price,
                    close: datum.close_price,
                }));
                candlestickSeries.setData(formattedData);
            });

        chart.timeScale().fitContent();
    });
});

function updateChartData() {
    fetch(`http://localhost:8000/data/${symbol}`)
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
}

function setupAutoRefresh() {
    // Call updateChartData initially
    updateChartData();

    // Set up the auto-refresh interval
    setInterval(updateChartData, resolution);
}

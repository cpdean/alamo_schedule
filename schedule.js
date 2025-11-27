// Global state
let scheduleData = null;
let showOnlyNextHour = false;
let searchQuery = '';
let timeFilterHours = 2;
let showOnlyCaptions = false;

// Format ISO datetime to readable format (full date and time)
function formatDateTime(isoString) {
    const date = new Date(isoString);
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    return `${month}/${day} ${hours}:${minutes}`;
}

// Format time only (HH:MM)
function formatTimeOnly(isoString) {
    const date = new Date(isoString);
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    return `${hours}:${minutes}`;
}

// Handle search input
function handleSearch() {
    const searchBox = document.getElementById('searchBox');
    searchQuery = searchBox.value.toLowerCase();
    displaySchedule();
}

// Handle time filter input
function handleTimeFilterChange() {
    const timeFilterInput = document.getElementById('timeFilterInput');
    const value = parseFloat(timeFilterInput.value);
    if (!isNaN(value) && value > 0) {
        timeFilterHours = value;
        displaySchedule();
    }
}

// Toggle between showing filtered time or all showtimes
function toggleFilter() {
    showOnlyNextHour = !showOnlyNextHour;
    displaySchedule();
}

// Toggle captions-only filter
function toggleCaptionsFilter() {
    showOnlyCaptions = !showOnlyCaptions;
    displaySchedule();
}

// Display the schedule based on current filter
function displaySchedule() {
    if (!scheduleData) return;
    
    const now = new Date();
    let filteredShowtimes;
    let startTime, endTime;
    
    if (showOnlyNextHour) {
        const filterEndTime = new Date(now.getTime() + timeFilterHours * 60 * 60 * 1000);
        filteredShowtimes = scheduleData.showtimes.filter(showtime => {
            const showTime = new Date(showtime.show_time);
            return showTime >= now && showTime <= filterEndTime;
        });
        startTime = formatDateTime(now.toISOString());
        endTime = formatDateTime(filterEndTime.toISOString());
    } else {
        // Filter out past shows - only show future shows
        filteredShowtimes = scheduleData.showtimes.filter(showtime => {
            const showTime = new Date(showtime.show_time);
            return showTime >= now;
        });
        startTime = formatDateTime(scheduleData.time_range.start);
        endTime = formatDateTime(scheduleData.time_range.end);
    }
    
    // Apply search filter
    if (searchQuery) {
        filteredShowtimes = filteredShowtimes.filter(showtime => 
            showtime.movie.toLowerCase().includes(searchQuery)
        );
    }
    
    // Apply captions-only filter
    if (showOnlyCaptions) {
        filteredShowtimes = filteredShowtimes.filter(showtime => showtime.open_caption);
    }
    
    // Update button text
    const toggleButton = document.getElementById('toggleButton');
    const hourText = timeFilterHours === 1 ? 'Hour' : 'Hours';
    toggleButton.textContent = showOnlyNextHour ? 'Show All Showtimes' : `Show Next ${timeFilterHours} ${hourText}`;
    
    // Update time range
    const timeRangeEl = document.getElementById('timeRange');
    timeRangeEl.textContent = `Showings between ${startTime} and ${endTime}`;
    
    // Display showtimes
    const showtimesEl = document.getElementById('showtimes');

    let tableRows;
    if (filteredShowtimes.length === 0) {
        let message;
        if (showOnlyCaptions) {
            message = 'No upcoming open caption showtimes in this range.';
        } else if (showOnlyNextHour) {
            message = 'No showtimes available in the selected time window.';
        } else {
            message = 'No upcoming showtimes found.';
        }

        tableRows = `
            <tr>
                <td class="no-results-cell" colspan="5">${message}</td>
            </tr>
        `;
    } else {
        tableRows = filteredShowtimes.map(showtime => {
        const formattedTime = formatTimeOnly(showtime.show_time);
        const captionStatus = showtime.open_caption ? '✓' : '—';
        const captionClass = showtime.open_caption ? 'caption-yes' : 'caption-no';
        
        // Calculate minutes until show
        const showTime = new Date(showtime.show_time);
        const minutesUntilShow = Math.round((showTime - now) / (1000 * 60));
        let rowClass = '';
        if (minutesUntilShow >= 0 && minutesUntilShow < 35) {
            rowClass = 'row-starting-soon';
        } else if (minutesUntilShow >= 35 && minutesUntilShow <= 60) {
            rowClass = 'row-upcoming';
        }
        
        // Format minutes display
        let minutesDisplay;
        if (minutesUntilShow < 0) {
            minutesDisplay = `${Math.abs(minutesUntilShow)}m ago`;
        } else {
            minutesDisplay = `${minutesUntilShow}m`;
        }
        
            return `
                <tr class="${rowClass}">
                    <td class="minutes-cell">${minutesDisplay}</td>
                    <td class="time-cell">${formattedTime}</td>
                    <td class="movie-cell">${showtime.movie}</td>
                    <td class="location-cell">${showtime.theater}</td>
                    <td class="caption-cell ${captionClass}">${captionStatus}</td>
                </tr>
            `;
        }).join('');
    }
    
    const captionsToggleLabel = showOnlyCaptions ? 'only captions' : 'any captions';
    const captionsHeaderClass = showOnlyCaptions ? 'captions-header captions-header-on' : 'captions-header captions-header-off';

    showtimesEl.innerHTML = `
        <table>
            <thead>
                <tr>
                    <th>In</th>
                    <th>Time</th>
                    <th>Movie</th>
                    <th>Location</th>
                    <th class="${captionsHeaderClass}" onclick="toggleCaptionsFilter()">
                        ${captionsToggleLabel}
                    </th>
                </tr>
            </thead>
            <tbody>
                ${tableRows}
            </tbody>
        </table>
    `;
}

// Load and display the schedule
async function loadSchedule() {
    try {
        const cacheBuster = Date.now();
        const response = await fetch(`current_schedule.json?v=${cacheBuster}`, {
            cache: 'no-store',
        });
        if (!response.ok) {
            throw new Error('Failed to load schedule');
        }
        
        scheduleData = await response.json();
        
        // Display the schedule with initial filter
        displaySchedule();
        
    } catch (error) {
        console.error('Error loading schedule:', error);
        document.getElementById('showtimes').innerHTML = `
            <div class="error">
                <strong>Error:</strong> Could not load schedule. Make sure current_schedule.json exists in the same directory.
            </div>
        `;
    }
}

// Load schedule when page loads
loadSchedule();

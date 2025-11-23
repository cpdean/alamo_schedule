// Global state
let scheduleData = null;
let showOnlyNextHour = false;
let searchQuery = '';

// Format ISO datetime to readable format
function formatDateTime(isoString) {
    const date = new Date(isoString);
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    return `${month}/${day} ${hours}:${minutes}`;
}

// Handle search input
function handleSearch() {
    const searchBox = document.getElementById('searchBox');
    searchQuery = searchBox.value.toLowerCase();
    displaySchedule();
}

// Toggle between showing next hour or all showtimes
function toggleFilter() {
    showOnlyNextHour = !showOnlyNextHour;
    displaySchedule();
}

// Display the schedule based on current filter
function displaySchedule() {
    if (!scheduleData) return;
    
    const now = new Date();
    let filteredShowtimes;
    let startTime, endTime;
    
    if (showOnlyNextHour) {
        const oneHourFromNow = new Date(now.getTime() + 60 * 60 * 1000);
        filteredShowtimes = scheduleData.showtimes.filter(showtime => {
            const showTime = new Date(showtime.show_time);
            return showTime >= now && showTime <= oneHourFromNow;
        });
        startTime = formatDateTime(now.toISOString());
        endTime = formatDateTime(oneHourFromNow.toISOString());
    } else {
        filteredShowtimes = scheduleData.showtimes;
        startTime = formatDateTime(scheduleData.time_range.start);
        endTime = formatDateTime(scheduleData.time_range.end);
    }
    
    // Apply search filter
    if (searchQuery) {
        filteredShowtimes = filteredShowtimes.filter(showtime => 
            showtime.movie.toLowerCase().includes(searchQuery)
        );
    }
    
    // Update button text
    const toggleButton = document.getElementById('toggleButton');
    toggleButton.textContent = showOnlyNextHour ? 'Show All Showtimes' : 'Show Next Hour Only';
    
    // Update time range
    const timeRangeEl = document.getElementById('timeRange');
    timeRangeEl.textContent = `Showings between ${startTime} and ${endTime}`;
    
    // Display showtimes
    const showtimesEl = document.getElementById('showtimes');
    
    if (filteredShowtimes.length === 0) {
        const message = showOnlyNextHour ? 'No showtimes available in the next hour.' : 'No showtimes available in this time range.';
        showtimesEl.innerHTML = `<div class="loading">${message}</div>`;
        return;
    }
    
    const tableRows = filteredShowtimes.map(showtime => {
        const formattedTime = formatDateTime(showtime.show_time);
        return `
            <tr>
                <td class="time-cell">${formattedTime}</td>
                <td class="movie-cell">${showtime.movie}</td>
                <td class="location-cell">${showtime.theater}</td>
            </tr>
        `;
    }).join('');
    
    showtimesEl.innerHTML = `
        <table>
            <thead>
                <tr>
                    <th>Time</th>
                    <th>Movie</th>
                    <th>Location</th>
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
        const response = await fetch('current_schedule.json');
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

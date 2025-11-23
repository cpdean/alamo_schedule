# Alamo Drafthouse Schedule Viewer

A web-based viewer for Alamo Drafthouse NYC movie showtimes with filtering capabilities.

## Files

### Core Application
- `src/main.rs` - Rust CLI tool that fetches and formats Alamo Drafthouse schedule data
- `index.html` - Web interface for viewing the schedule
- `schedule.js` - JavaScript for rendering and filtering showtimes
- `current_schedule.json` - JSON data file containing showtime information

### Automation
- `.github/workflows/generate-schedule.yml` - GitHub Actions workflow that automatically updates the schedule

## How It Works

### Data Generation

The Rust CLI tool fetches schedule data from the Alamo Drafthouse API and outputs it in two formats:

```bash
# Generate text output (default)
cargo run --bin alamo_schedule

# Generate JSON output
cargo run --bin alamo_schedule -- --output json
```

The JSON output contains:
- Time range (start and end times)
- Array of showtimes with:
  - Show time
  - Movie title
  - Theater location
  - Cinema ID
  - Session ID
  - Presentation slug

### Updating the Schedule

#### Manual Update

To manually update the schedule data:

```bash
# Generate new schedule JSON
cargo run --bin alamo_schedule -- --output json > current_schedule.json
```

#### Automatic Update

The GitHub Actions workflow automatically:
1. Builds the Rust application
2. Runs the tool with JSON output
3. Commits the updated `current_schedule.json` file
4. Pushes changes to the repository

The workflow can be triggered manually from the GitHub Actions tab.

## Viewing the Web Interface

**IMPORTANT:** The web interface must be served through a local HTTP server to properly load the JSON data file. Opening `index.html` directly in a browser (`file://` protocol) will fail due to CORS security restrictions.

### Local Viewing

Use the provided script to start the server:

```bash
./serve.sh
```

This will display URLs for accessing the page from:
- This device: `http://localhost:8000`
- Other devices on the same network: `http://[your-ip]:8000`

Alternatively, start the Python server manually:

```bash
python3 -m http.server 8000
```

Then open your browser to: `http://localhost:8000`

## Web Interface Features

- **Table View**: Displays showtimes in a clean, sortable table format
- **Time Filter**: Toggle between showing all showtimes or only those in the next hour
- **Responsive Design**: Works on desktop and mobile devices
- **Auto-refresh**: Recalculates time-based filters on each toggle

### Columns

- **Time**: Showtime in MM/DD HH:MM format
- **Movie**: Film title
- **Location**: Theater name (excludes Staten Island)

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Modifying the Web Interface

1. Edit `index.html` for structure and styles
2. Edit `schedule.js` for functionality
3. Test changes using a local HTTP server
4. Commit changes to git

### Modifying the CLI Tool

1. Edit `src/main.rs` for CLI functionality
2. Edit `src/data.rs` for data structures
3. Build and test with `cargo run`
4. Update `current_schedule.json` with new output
5. Commit changes to git

## Project Structure

```
alamo_schedule/
├── .github/
│   └── workflows/
│       └── generate-schedule.yml    # Auto-update workflow
├── src/
│   ├── main.rs                      # CLI application
│   ├── data.rs                      # Data structures
│   └── bin/
│       └── open_caption.rs          # Open caption filter tool
├── index.html                       # Web viewer
├── schedule.js                      # Web viewer logic
├── current_schedule.json            # Schedule data
├── serve.sh                         # Helper script to start server
└── README.md                        # This file
```

## Notes

- The schedule shows movies for the next 12 hours by default (from the CLI)
- Staten Island theater is excluded from results
- The web interface can filter to show only the next hour
- All times are displayed in local timezone

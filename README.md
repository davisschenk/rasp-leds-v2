# Raspberry PI & WS2812b Led Controller
This project is a rewrite of my original [LED Controller](https://github.com/davisschenk/rasp_leds) with the aim to improve the code quality and implement the ability to simulate the led strip virtually. The implementation uses rocket to create a REST API that can be used to control a strip of WS2812b leds and run a variety of patterns using a raspberry pi.

## End Points
The API currently supports a number of endpoints
- POST /on
Turns on the strip, IE runs the last pattern, if no last pattern returns an error
- POST /off
Turns off the strip
- POST /power
Toggles between on and off
- POST /pattern
Runs one of the patterns
- GET /history
Returns a JSON list of the previous patterns
- GET /info
Return an object with the led count, state and current leds

## Hardware
- Raspberry Pi 3b+
- Custom PCB
- ws2812b 10m LED Strip

## Future Plans
- Spotify Integration
- More patterns
- Fancy front end
- Improved hardware
3d printed enclosure, possibly use a PI Zero 2W, hardware on/off
- Actual documentation

# Steering Wheel Controller Firmware

## Overview

This firmware is designed to control a steering wheel setup with pedals and transmission controls based on the electronic board 'Arduino Leonardo'. It uses the [HID](https://github.com/NicoHood/HID) (Human Interface Device) library to communicate with the computer as a gamepad.

The full gamepad API from the [HID](https://github.com/NicoHood/HID) library, you can find [here](https://github.com/NicoHood/HID/wiki/Gamepad-API).

## Preferences

### Steering Wheel:
* STEERING_WHEEL_PIN: Analog pin for the steering wheel potentiometer
* STEERING_WHEEL_DEAD_ZONE: Array defining the dead zone values
* STEERING_WHEEL_PIN_MAX_VALUE: Maximum possible value of the your steering wheel potentiometer
* STEERING_WHEEL_PIN_CENTER_VALUE: Center value calculated as half of max value
* STEERING_WHEEL_MAX_VALUE: Maximum value considering dead zone
* STEERING_WHEEL_STICK_COOF: Coefficient for scaling the joystick values (see the [gamepad API](https://github.com/NicoHood/HID/wiki/Gamepad-API))

### Transmission & Handbrake:
* TRANSMISSION_UP_PIN: Digital pin for the transmission up button
* TRANSMISSION_DOWN_PIN: Digital pin for the transmission down button
* HANDBRAKE_PIN: Digital pin for the handbrake button

### Pedals:
* PEDAL_GAS_PIN: Analog pin for the gas pedal potentiometer
* PEDAL_BRAKE_PIN: Analog pin for the brake pedal potentiometer
* PEDAL_CLUTCH_PIN: Analog pin for the clutch pedal potentiometer
* PEDAL.._DEAD_ZONE: Constants defining dead zones for each pedals
* PEDAL.._PIN_MAX_VALUE: Maximum possible value of the your pedal potentiometers
* PEDAL.._STICK_COOF: Coefficient for scaling the joystick values (see the [gamepad API](https://github.com/NicoHood/HID/wiki/Gamepad-API))

### Data Tracking:
* .._value & .._last_value: Each input has variables to track current and previous states
* gamepad_have_changes: Flag to indicate if any changes occurred in the gamepad state

### Configuration Notes:
* All analog readings are scaled to fit within the standard gamepad input range
* Dead zones are implemented to ignore minor movements or readings
* Variables marked as volatile are used for values that can change during interrupts

## Usage
This firmware reads inputs from the steering wheel, pedals, and buttons, applies necessary scaling and dead zone adjustments, and can be configured to send these values as gamepad inputs. The gamepad_have_changes flag is used to optimize data transmission by only sending updates when necessary.

# Licensing
Distributed under the MIT license.

# Feedback
You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This project is constantly evolving, and I welcome your suggestions and feedback.

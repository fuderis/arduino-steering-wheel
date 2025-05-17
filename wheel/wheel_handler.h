const float WHEEL_DEAD_ZONE_VALUE = 1020 * WHEEL_DEAD_ZONE / 100;

const int WHEEL_CENTER_VALUE = 1020 / 2;
const float WHEEL_DEGS_COOF = (float)WHEEL_CENTER_VALUE / (float)WHEEL_DEGS_MAX;
const int WHEEL_MAX_VALUE = WHEEL_DEGS_LIMIT * WHEEL_DEGS_COOF;
const float WHEEL_STICK_COOF = 32767 / (float)WHEEL_MAX_VALUE;

volatile float wheel_last_value = 0.0;
volatile float wheel_value = 0.0;
volatile bool wheel_direction = false;  // false: to left, true: to right

// The steering wheel handler
void wheel_handler() {    
    int value = analogRead(WHEEL_PIN) - WHEEL_CENTER_VALUE;

    // check dead zone (center):
    if (abs(value) < WHEEL_DEAD_ZONE_VALUE) {
        wheel_value = 0;
    }
    // check dead zone (right):
    else if (value >= WHEEL_MAX_VALUE) {
        wheel_value = WHEEL_MAX_VALUE;
    }
    // check dead zone (left):
    else if (value <= -WHEEL_MAX_VALUE) {
        wheel_value = -WHEEL_MAX_VALUE;
    }
    // aplying a value:
    else {
        wheel_value = value - (value > 0 ? WHEEL_DEAD_ZONE_VALUE : -WHEEL_DEAD_ZONE_VALUE);
    }

    if (abs(wheel_value - wheel_last_value) > 1 || (wheel_value == 0 && wheel_last_value != 0)) {
        Gamepad.xAxis(wheel_value * WHEEL_STICK_COOF);
        Gamepad.write();

        Serial.print("Wheel value: ");
        Serial.println(wheel_value);

        if (wheel_value < wheel_last_value - 10) {
            wheel_direction = false;
        } else if (wheel_value > wheel_last_value + 10) {
            wheel_direction = true;
        }

        wheel_last_value = wheel_value;
    }
}

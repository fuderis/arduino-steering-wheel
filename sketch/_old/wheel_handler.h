const float WHEEL_DEAD_ZONE_VALUE = 1020 * WHEEL_DEAD_ZONE / 100;

const int WHEEL_CENTER_VALUE = 1020 / 2;
const float WHEEL_DEGS_COOF = (float)WHEEL_CENTER_VALUE / (float)WHEEL_DEGS_MAX;
const int WHEEL_MAX_VALUE = WHEEL_DEGS_LIMIT * WHEEL_DEGS_COOF;
const float WHEEL_STICK_COOF = 32767 / (float)WHEEL_MAX_VALUE;

volatile float wheel_value = 0.0;
volatile bool wheel_direction = false;  // false: to left, true: to right

// The steering wheel handler
void wheel_handler() {    
    int value = analogRead(WHEEL_PIN) - WHEEL_CENTER_VALUE;

    // check dead zone (center):
    if (abs(value) <= WHEEL_DEAD_ZONE_VALUE) {
        wheel_value = 0;
    } else if (value != wheel_value) {
        Gamepad.xAxis( constrain(wheel_value, -WHEEL_MAX_VALUE, WHEEL_MAX_VALUE) * WHEEL_STICK_COOF);
        Gamepad.write();

        Serial.print("Wheel value: ");
        Serial.println(wheel_value);

        wheel_value = value;
        wheel_direction = wheel_value > 0;
    }
}

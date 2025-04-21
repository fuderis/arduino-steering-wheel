const int WHEEL_CENTER_VALUE = 1024 / 2;
const int WHEEL_MAX_VALUE = WHEEL_CENTER_VALUE - WHEEL_DEAD_ZONE[1];
const float WHEEL_STICK_COOF = 32767 / (float)WHEEL_MAX_VALUE;

volatile float wheel_last_value = 0.0;
volatile float wheel_value = 0.0;

// The steering wheel handler
void wheel_handler() {
    int value = -1 * (analogRead(WHEEL_PIN) - WHEEL_CENTER_VALUE);

    // check dead zone (center):
    if (abs(value) < WHEEL_DEAD_ZONE[0]) {
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
        wheel_value = value - (value > 0 ? WHEEL_DEAD_ZONE[0] : -WHEEL_DEAD_ZONE[0]);
    }

    if (wheel_value != wheel_last_value) {
        Gamepad.xAxis(wheel_value * WHEEL_STICK_COOF);
        Gamepad.write();

        Serial.print("Wheel value: ");
        Serial.println(wheel_value);

        wheel_last_value = wheel_value;
    }
}

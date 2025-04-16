// The steering wheel handler
void steering_wheel_handler() {
    int value = analogRead(STEERING_WHEEL_PIN) - STEERING_WHEEL_CENTER_VALUE;

    // check dead zone (center):
    if (abs(value) < STEERING_WHEEL_DEAD_ZONE[0]) {
        steering_wheel_value = 0;
    }
    // check dead zone (right):
    else if (value >= STEERING_WHEEL_MAX_VALUE) {
        steering_wheel_value = STEERING_WHEEL_MAX_VALUE;
    }
    // check dead zone (left):
    else if (value <= -STEERING_WHEEL_MAX_VALUE) {
        steering_wheel_value = -STEERING_WHEEL_MAX_VALUE;
    }
    // aplying a value:
    else {
        steering_wheel_value = value - (value > 0 ? STEERING_WHEEL_DEAD_ZONE[0] : -STEERING_WHEEL_DEAD_ZONE[0]);
    }

    if (steering_wheel_value != steering_wheel_last_value) {
        Gamepad.xAxis(steering_wheel_value * STEERING_WHEEL_STICK_COOF);
        Gamepad.write();

        Serial.print("Steering wheel value: ");
        Serial.println(steering_wheel_value);

        steering_wheel_last_value = steering_wheel_value;
    }
}

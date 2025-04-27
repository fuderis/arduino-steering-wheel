const float WHEEL_FEEDBACK_MOVE_COOF = (WHEEL_FEEDBACK_MAX_SPEED - WHEEL_FEEDBACK_MIN_SPEED) / (float)WHEEL_MAX_VALUE;
volatile int wheel_feedback_speed = 0;
volatile bool wheel_feedback_direction = false;   // false: to left, true: to right

// Stops the feedback motor
void wheel_feedback_stop() {
    wheel_feedback_speed = 0;
    
    analogWrite(WHEEL_FEEDBACK_ENA_PIN, LOW);
    analogWrite(WHEEL_FEEDBACK_IN1_PIN, LOW);
    analogWrite(WHEEL_FEEDBACK_IN2_PIN, LOW);
}

// Moves the feedback motor (direction [false: to left, true: to right])
void wheel_feedback_move(bool direction, int speed, int time) {
    // set speed:
    wheel_feedback_speed = speed > 0 ? speed : abs(wheel_value) * WHEEL_FEEDBACK_MOVE_COOF + WHEEL_FEEDBACK_MIN_SPEED;
    analogWrite(WHEEL_FEEDBACK_ENA_PIN, wheel_feedback_speed);

    Serial.print("Feedback speed: ");
    Serial.println(wheel_feedback_speed);

    // set direction:
    digitalWrite(WHEEL_FEEDBACK_IN1_PIN, direction ? HIGH : LOW);
    digitalWrite(WHEEL_FEEDBACK_IN2_PIN, direction ? LOW : HIGH);

    // waiting time..:
    if (time > 0) {
        delay(time);
        wheel_feedback_stop();
    }
}

// The steering wheel feedback motor handler
void wheel_feedback_handler() {
    if (wheel_value < WHEEL_FEEDBACK_DEAD_ZONE && wheel_value > -WHEEL_FEEDBACK_DEAD_ZONE) {
        wheel_feedback_stop();
    } else {
        wheel_feedback_direction = wheel_value < 0;

        wheel_feedback_move(wheel_feedback_direction, 0, 0);

        if (wheel_feedback_speed == 0) {
            Serial.print("Feedback direction: ");
            Serial.println(wheel_feedback_direction ? "to right" : "to left");
        }
    }
}

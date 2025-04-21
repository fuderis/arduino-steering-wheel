const float WHEEL_FEEDBACK_MOVE_COOF = (WHEEL_FEEDBACK_MAX_SPEED - WHEEL_FEEDBACK_MIN_SPEED) / (float)WHEEL_MAX_VALUE;
volatile int wheel_feedback_speed = 0;
volatile bool wheel_feedback_direction = false;   // false: to left, true: to right

void wheel_feedback_set_speed(int speed) {
    wheel_feedback_speed = speed;

    analogWrite(WHEEL_FEEDBACK_ENA_PIN, speed);
}

// Sets the minimum feedback motor moving speed
void wheel_feedback_set_min_speed() {
    wheel_feedback_set_speed(WHEEL_FEEDBACK_MIN_SPEED);
}

// Sets the maximum feedback motor moving speed
void wheel_feedback_set_max_speed() {
    wheel_feedback_set_speed(WHEEL_FEEDBACK_MAX_SPEED);
}

// Sets the feedback motor moving speed
void wheel_feedback_set_auto_speed() {
    float power = abs(wheel_value) * WHEEL_FEEDBACK_MOVE_COOF + WHEEL_FEEDBACK_MIN_SPEED;

    wheel_feedback_set_speed(power);
}

// Sets the feedback motor move direction (false: to left, true: to right)
void wheel_feedback_set_direction(bool direction) {
    digitalWrite(WHEEL_FEEDBACK_IN1_PIN, direction ? HIGH : LOW);
    digitalWrite(WHEEL_FEEDBACK_IN2_PIN, direction ? LOW : HIGH);
}

// Stops the feedback motor moving
void wheel_feedback_stop() {
    wheel_feedback_speed = 0;
    
    analogWrite(WHEEL_FEEDBACK_ENA_PIN, 0);
    digitalWrite(WHEEL_FEEDBACK_IN1_PIN, LOW);
    digitalWrite(WHEEL_FEEDBACK_IN2_PIN, LOW);
}

// Moves the feedback motor by direction & time
void wheel_feedback_move(bool direction, int time) {
    wheel_feedback_set_min_speed();
    wheel_feedback_set_direction(direction);

    delay(time);

    wheel_feedback_stop();
}

// Centralizes the steering wheel position
void wheel_feedback_centralize() {
    while (wheel_value > WHEEL_FEEDBACK_DEAD_ZONE || wheel_value < -WHEEL_FEEDBACK_DEAD_ZONE) {
        wheel_feedback_set_auto_speed();
        wheel_feedback_set_direction(wheel_value < 0);

        delay(50);
    }

    wheel_feedback_stop();
}

// The steering wheel feedback motor handler
void wheel_feedback_handler() {
    if (wheel_value < WHEEL_FEEDBACK_DEAD_ZONE && wheel_value > -WHEEL_FEEDBACK_DEAD_ZONE) {
        wheel_feedback_stop();
    } else {
        wheel_feedback_direction = wheel_value < 0;

        wheel_feedback_set_auto_speed();
        wheel_feedback_set_direction(wheel_feedback_direction);

        if (wheel_feedback_speed == 0) {
            Serial.print("Feedback direction: ");
            Serial.println(wheel_feedback_direction ? "to right" : "to left");
        }
    }
}

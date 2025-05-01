const int FEEDBACK_DEAD_ZONE_VALUE = WHEEL_MAX_VALUE * FEEDBACK_DEAD_ZONE / 100;
const float FEEDBACK_MOVE_COOF = (FEEDBACK_MAX_SPEED - FEEDBACK_MIN_SPEED) / (float)WHEEL_MAX_VALUE;

const float FEEDBACK_MIN_SPEED_VALUE = 255 * FEEDBACK_MIN_SPEED / 100;
const float FEEDBACK_MAX_SPEED_VALUE = 255 * FEEDBACK_MAX_SPEED / 100;

volatile int feedback_speed = 0;
volatile int last_feedback_speed = 0;
volatile bool feedback_direction = false;  // false: to left, true: to right

// Stops the feedback motor
void feedback_stop() {
    feedback_speed = 0;

    // braking engine:
    pinMode(FEEDBACK_IN1_PIN, OUTPUT);
    analogWrite(FEEDBACK_IN1_PIN, FEEDBACK_MAX_SPEED_VALUE);

    pinMode(FEEDBACK_IN2_PIN, OUTPUT);
    analogWrite(FEEDBACK_IN2_PIN, FEEDBACK_MAX_SPEED_VALUE);
    delay(5);
    
    // stopping engine:
    pinMode(FEEDBACK_IN1_PIN, INPUT);
    digitalWrite(FEEDBACK_IN1_PIN, LOW);

    pinMode(FEEDBACK_IN2_PIN, INPUT);
    digitalWrite(FEEDBACK_IN2_PIN, LOW);

    if (last_feedback_speed != 0) {
        Serial.println("Feedback stopped");
        last_feedback_speed = 0;
    }
}

// Moves the feedback motor (direction [false: to left, true: to right])
void feedback_move(bool direction, int speed, int time) {
    // set speed:
    feedback_speed = speed > 0 ? speed : abs(wheel_value) * FEEDBACK_MOVE_COOF + FEEDBACK_MIN_SPEED_VALUE;
    // analogWrite(FEEDBACK_ENA_PIN, feedback_speed);

    // set direction to right:
    if (direction) {
        pinMode(FEEDBACK_IN1_PIN, OUTPUT);
        analogWrite(FEEDBACK_IN1_PIN, feedback_speed);

        pinMode(FEEDBACK_IN2_PIN, INPUT);
        digitalWrite(FEEDBACK_IN2_PIN, LOW);
    }
    // set direction to left:
    else {
        pinMode(FEEDBACK_IN1_PIN, INPUT);
        digitalWrite(FEEDBACK_IN1_PIN, LOW);

        pinMode(FEEDBACK_IN2_PIN, OUTPUT);
        analogWrite(FEEDBACK_IN2_PIN, feedback_speed);
    }

    if (feedback_speed != last_feedback_speed) {        
        Serial.print("Feedback moves ");
        Serial.print(feedback_direction ? "to right" : "to left");
        Serial.print(" by speed ");
        Serial.println(feedback_speed);

        last_feedback_speed = feedback_speed;
    }

    // waiting time..:
    if (time > 0) {
        delay(time);
        feedback_stop();
    }
}

// The steering wheel feedback motor handler
void feedback_handler() {
    if (wheel_value < -FEEDBACK_DEAD_ZONE_VALUE || wheel_value > FEEDBACK_DEAD_ZONE_VALUE) {
        feedback_direction = wheel_value < 0;
        
        if (feedback_direction == wheel_direction) {
            feedback_move(feedback_direction, 0, 0);
        } else {
            feedback_move(feedback_direction, FEEDBACK_MIN_SPEED_VALUE, 0);   // set the minimum engine speed here to avoid overheating
        }
    } else if (feedback_speed > 0) {
        feedback_stop();
    }
}

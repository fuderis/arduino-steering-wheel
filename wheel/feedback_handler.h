const float FEEDBACK_DEAD_ZONE_VALUE = WHEEL_MAX_VALUE * FEEDBACK_DEAD_ZONE / 100;

const float FEEDBACK_MIN_SPEED_VALUE = PWM_MAX_VALUE * FEEDBACK_MIN_SPEED / 100;
const float FEEDBACK_MAX_SPEED_VALUE = PWM_MAX_VALUE * FEEDBACK_MAX_SPEED / 100;

const float FEEDBACK_MOVE_COOF = (float)(FEEDBACK_MAX_SPEED_VALUE - FEEDBACK_MIN_SPEED_VALUE) / (float)WHEEL_MAX_VALUE;

volatile bool feedback_direction = false;  // [false: to left, true: to right]
volatile float feedback_speed = 0;
volatile float last_feedback_speed = 0;

// Set feedback motor speed
void feedback_set_speed(float speed) {
    speed = (int)constrain(speed, FEEDBACK_MIN_SPEED_VALUE, FEEDBACK_MAX_SPEED_VALUE);
    analogWrite9(speed);
    analogWrite10(speed);

    feedback_speed = speed;
}

// Set feedback motor direction [false: to left, true: to right]
void feedback_set_direction(bool direct) {
    if (direct) {
        pinMode(FEEDBACK_PWM_L_PIN, OUTPUT);
        digitalWrite(FEEDBACK_PWM_L_PIN, HIGH);

        pinMode(FEEDBACK_PWM_R_PIN, INPUT);
        digitalWrite(FEEDBACK_PWM_R_PIN, LOW);
    } else {
        pinMode(FEEDBACK_PWM_L_PIN, INPUT);
        digitalWrite(FEEDBACK_PWM_L_PIN, LOW);

        pinMode(FEEDBACK_PWM_R_PIN, OUTPUT);
        digitalWrite(FEEDBACK_PWM_R_PIN, HIGH);
    }

    feedback_direction = direct;
}

// Start feedback motor braking
void feedback_brake() {
    feedback_set_speed(FEEDBACK_MAX_SPEED_VALUE);
    
    pinMode(FEEDBACK_PWM_L_PIN, OUTPUT);
    digitalWrite(FEEDBACK_PWM_L_PIN, HIGH);

    pinMode(FEEDBACK_PWM_R_PIN, OUTPUT);
    digitalWrite(FEEDBACK_PWM_R_PIN, HIGH);
}

// Start feedback motor free running
void feedback_free() {
    feedback_set_speed(0);
    
    pinMode(FEEDBACK_PWM_L_PIN, INPUT);
    digitalWrite(FEEDBACK_PWM_L_PIN, LOW);

    pinMode(FEEDBACK_PWM_R_PIN, INPUT);
    digitalWrite(FEEDBACK_PWM_R_PIN, LOW);
}

// Stops the feedback motor
void feedback_stop() {
    feedback_brake();
    delay(10);
    feedback_free();

    if (last_feedback_speed != 0) {
        Serial.println("Feedback stopped");
        last_feedback_speed = 0;
    }
}

// Moves the feedback motor (direction [false: to left, true: to right])
void feedback_move(bool direct, int speed, int time) {
    feedback_set_speed(speed > 0 ? speed : (int)(abs(wheel_value) * FEEDBACK_MOVE_COOF + FEEDBACK_MIN_SPEED_VALUE));
    feedback_set_direction(direct);

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
        
        feedback_move(feedback_direction, 0, 0);

        // if (feedback_direction == wheel_direction) {
        //     feedback_move(feedback_direction, 0, 0);
        // } else {
        //     feedback_move(feedback_direction, FEEDBACK_MIN_SPEED_VALUE, 0);   // set minimum speed to avoid overheating
        // }
    } else if (feedback_speed > 0) {
        feedback_stop();
    }
}

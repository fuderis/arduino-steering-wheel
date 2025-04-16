// The steering wheel feedback motor handler
void steering_wheel_feedback_handler() {
    if (steering_wheel_value < STEERING_WHEEL_FEEDBACK_DEAD_ZONE && steering_wheel_value > -STEERING_WHEEL_FEEDBACK_DEAD_ZONE) {
        digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, LOW);
        digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, LOW);
    } else if (steering_wheel_value < 0) {
        float power = abs(steering_wheel_value) * STEERING_WHEEL_FEEDBACK_COOF + STEERING_WHEEL_FEEDBACK_MIN_POWER;
        analogWrite(STEERING_WHEEL_FEEDBACK_ENA_PIN, power);

        digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, HIGH);
        digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, LOW);

        Serial.print("Feedback is powered to right: ");
        Serial.println(power);
    }
    else {
        float power = abs(steering_wheel_value) * STEERING_WHEEL_FEEDBACK_COOF + STEERING_WHEEL_FEEDBACK_MIN_POWER;
        analogWrite(STEERING_WHEEL_FEEDBACK_ENA_PIN, power);

        digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, LOW);
        digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, HIGH);

        Serial.print("Feedback is powered to left: ");
        Serial.println(power);
    }
}

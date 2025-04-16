void demonstrative_start() {
  analogWrite(STEERING_WHEEL_FEEDBACK_ENA_PIN, STEERING_WHEEL_FEEDBACK_MIN_POWER);

  // move to left
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, LOW);
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, HIGH);
  delay(1000);

  // move to right
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, HIGH);
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, LOW);
  delay(1000);

  // stop
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN1_PIN, LOW);
  digitalWrite(STEERING_WHEEL_FEEDBACK_IN2_PIN, LOW);
}

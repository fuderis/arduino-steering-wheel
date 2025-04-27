void demonstration() {
    wheel_feedback_move(false, WHEEL_FEEDBACK_MIN_SPEED, 500);
    delay(600);

    wheel_feedback_move(true, WHEEL_FEEDBACK_MIN_SPEED, 700);
    delay(600);
}

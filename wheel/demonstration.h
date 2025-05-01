// Centralizes the wheel position
void demonstration_centralize() {
    while (wheel_value < -5 || wheel_value > 5) {
        feedback_direction = wheel_value < 0;
        feedback_move(feedback_direction, FEEDBACK_MIN_SPEED_VALUE, 0);
    }

    feedback_stop();
}

// The steering wheel demonstrative starting
void demonstration() {
    demonstration_centralize();
    delay(100);
    
    feedback_move(false, FEEDBACK_MIN_SPEED_VALUE, 500);
    delay(600);

    feedback_move(true, FEEDBACK_MIN_SPEED_VALUE, 700);
    delay(600);

    demonstration_centralize();
}

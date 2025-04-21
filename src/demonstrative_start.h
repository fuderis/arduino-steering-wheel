void demonstrative_start() {
    wheel_feedback_centralize();

    wheel_feedback_move(false, 300);
    delay(600);

    wheel_feedback_centralize();

    wheel_feedback_move(true, 400);
    delay(600);

    wheel_feedback_centralize();
}

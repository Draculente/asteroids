//! This module contains the ErrorMessage struct. This struct is used to display error messages to the user. It contains methods to dismiss this error message after a certain amount of time and a macro to simplify the creation of error messages.

/// This error macro is used facilitate to creation of a new error message. Macros allow for default values of arguments and are easier to use.
#[macro_export]
macro_rules! error {
    ($a: expr) => {
        ErrorMessage::new_dismissable($a, 100)
    };
    ($a: expr, $b: expr) => {
        ErrorMessage::new_dismissable($a, $b)
    };
}

/// The ErrorMessage struct is used to display error messages to the user.
#[derive(Debug)]
pub struct ErrorMessage {
    /// The message of the error. This is a string.
    message: Option<&'static str>,
    /// Whether the error message is dismissable. If it is dismissable, the user can click on the error message to dismiss it.
    dismissable: bool,
    /// The time since the error message was created. This is used to dismiss the error message after a certain amount of time.
    time_since_creation: i32,
    /// The duration of the error message. This is used to dismiss the error message after a certain amount of time.
    message_duration: i32,
}

impl ErrorMessage {
    /// Creates a new empty error message. This is used to initialize the error message.
    pub fn new_empty() -> Self {
        Self {
            message: None,
            dismissable: false,
            time_since_creation: 0,
            message_duration: 0,
        }
    }

    /// Creates a new error message. This is used to create an error message with a message and a duration.
    pub fn new_dismissable(message: &'static str, duration: i32) -> Self {
        Self {
            message: Some(message),
            dismissable: false,
            time_since_creation: 0,
            message_duration: duration,
        }
    }

    /// Creates a new error message. This is used to create an error message with a message and a duration.
    pub fn new_non_dismissable(message: &'static str, duration: i32) -> Self {
        Self {
            message: Some(message),
            dismissable: true,
            time_since_creation: 0,
            message_duration: duration,
        }
    }

    /// Ticks the error message. This is used to dismiss the error message after a certain amount of time.
    pub fn tick(&mut self) {
        if self.is_expired() {
            self.message = None;
        } else {
            self.time_since_creation += 1;
        }
    }

    /// Returns whether the error message is expired. This is used to dismiss the error message after a certain amount of time.
    pub fn is_expired(&self) -> bool {
        self.time_since_creation > self.message_duration
    }

    /// Returns the message of the error message. This is used to display the error message.
    pub fn get_message(&self) -> Option<String> {
        if !self.is_expired() {
            self.message.map(|s| s.to_string())
        } else {
            None
        }
    }

    /// Sets the message of the error message.
    pub fn set_error_message(&mut self, message: &'static str) {
        self.message = Some(message);
        self.time_since_creation = 0;
    }

    /// Dismisses the error message.
    pub fn dismiss(&mut self) {
        self.message = None;
    }
}

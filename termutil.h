#pragma once

extern "C" {
	void tu_pause();
	void tu_pause_with_message(const char *msg);
	void tu_clear();
	void tu_sleep(unsigned int millis);
}

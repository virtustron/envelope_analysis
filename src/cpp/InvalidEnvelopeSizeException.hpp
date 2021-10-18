#pragma once

#include <string>

const int UNDEFINED_ENVELOPE_SIZE = -1;

class InvalidEnvelopeSizeException
{
public:
	InvalidEnvelopeSizeException();
	InvalidEnvelopeSizeException(const std::string error_message);
	InvalidEnvelopeSizeException(const std::string error_message, int size_value);

	std::string get_error_message() const;
	int get_size_value();

private:
	std::string m_error_message;
	int m_size_value;

};


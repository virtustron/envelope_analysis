#include "InvalidEnvelopeSizeException.hpp"

InvalidEnvelopeSizeException::InvalidEnvelopeSizeException()
{
	m_error_message = "";
	m_size_value = 0;
}

InvalidEnvelopeSizeException::InvalidEnvelopeSizeException(const std::string error_message)
{
	m_error_message = error_message;
	m_size_value = UNDEFINED_ENVELOPE_SIZE;
}

InvalidEnvelopeSizeException::InvalidEnvelopeSizeException(const std::string error_message, int size_value)
{
	m_error_message = error_message;
	m_size_value = size_value;
}

std::string InvalidEnvelopeSizeException::get_error_message() const
{
	return m_error_message;
}

int InvalidEnvelopeSizeException::get_size_value()
{
	return m_size_value;
}

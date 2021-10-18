#include "Envelope.hpp"



Envelope::Envelope()
{
	m_long_side_size = 1;
	m_short_side_size = 1;
}




Envelope::Envelope(double size_1, double size_2)
{
	set_side_sizes(size_1, size_2);
}









double Envelope::get_long_side_size() const
{
	return m_long_side_size;
}

double Envelope::get_short_side_size() const 
{
	return m_short_side_size;
}

void Envelope::set_side_sizes(char* size_1, char* size_2)
{
	double converted_size_1 = UNDEFINED_ENVELOPE_SIZE;
	double converted_size_2 = UNDEFINED_ENVELOPE_SIZE;

	converted_size_1 = atof(size_1);
	if (converted_size_1 == ATOF_NO_CONVERSION_CAN_BE_PERFORMED)
	{
		throw InvalidEnvelopeSizeException("Conversion of envelope size #1 value can't be performed");
	}

	converted_size_2 = atof(size_2);
	if (converted_size_2 == ATOF_NO_CONVERSION_CAN_BE_PERFORMED)
	{
		throw InvalidEnvelopeSizeException("Conversion of envelope size #2 value can't be performed");
	}

	set_side_sizes(converted_size_1, converted_size_2);
}


void Envelope::set_side_sizes(double size_1, double size_2)
{
	if (!IsSizeValid(size_1))
		throw InvalidEnvelopeSizeException("Invalid envelope size value", size_1);

	if (!IsSizeValid(size_2))
		throw InvalidEnvelopeSizeException("Invalid envelope size value", size_2);

	if (size_1 > size_2)
	{
		m_long_side_size = size_1;
		m_short_side_size = size_2;
	}
	else
	{
		m_long_side_size = size_2;
		m_short_side_size = size_1;
	}
}

bool Envelope::IsSizeValid(double envelope_size)
{
	if (envelope_size <= 0)
		return false;

	return true;
}
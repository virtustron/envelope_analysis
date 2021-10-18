#pragma once

#include <cstdlib>
#include "InvalidEnvelopeSizeException.hpp"

const double ATOF_NO_CONVERSION_CAN_BE_PERFORMED = 0.0;

class Envelope
{
public:
	Envelope();
	Envelope(double side_1, double side_2);
	
	double get_long_side_size() const;
	double get_short_side_size() const;
	void set_side_sizes(char* size_1, char* size_2);
	void set_side_sizes(double size_1, double size_2);

private:
	double m_long_side_size;
	double m_short_side_size;
	
	bool IsSizeValid(double envelope_size);
};


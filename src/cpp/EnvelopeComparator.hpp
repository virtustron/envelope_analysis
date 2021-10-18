#pragma once
#include "Envelope.hpp"

class EnvelopeComparator
{
public:
	static bool CanOneContainAnother(Envelope* envelope_1, Envelope* envelope_2);
};


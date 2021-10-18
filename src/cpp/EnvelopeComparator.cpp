#include "EnvelopeComparator.hpp"

bool EnvelopeComparator::CanOneContainAnother(Envelope* envelope_1, Envelope* envelope_2)
{
	if (envelope_1->get_long_side_size() > envelope_2->get_long_side_size() &&
		envelope_1->get_short_side_size() > envelope_2->get_short_side_size())
	{
		return true;
	}
	else if (envelope_1->get_long_side_size() < envelope_2->get_long_side_size() &&
		envelope_1->get_short_side_size() < envelope_2->get_short_side_size())
	{
		return true;
	} 
	else
	{
		return false;
	}

}
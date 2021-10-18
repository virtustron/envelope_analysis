#pragma ones

#include "Envelope.hpp"

class EnvelopesContainer
{
public:
    EnvelopesContainer();
    EnvelopesContainer(double, double, double, double);
    ~EnvelopesContainer();

    Envelope* get_envelope_1();
    Envelope* get_envelope_2();
private:
    Envelope* m_envelope_1;
    Envelope* m_envelope_2;
};

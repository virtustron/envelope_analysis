#include "EnvelopesContainer.hpp"

EnvelopesContainer::EnvelopesContainer()
{
    m_envelope_1 = new Envelope();
    m_envelope_2 = new Envelope();
}

EnvelopesContainer::EnvelopesContainer(double envelope1_size1, double envelope1_size2, double envelope2_size1, double envelope2_size2) : EnvelopesContainer()
{
    m_envelope_1->set_side_sizes(envelope1_size1,envelope1_size2);
    m_envelope_2->set_side_sizes(envelope2_size1,envelope2_size2);
}

EnvelopesContainer::~EnvelopesContainer()
{
    delete m_envelope_1;
    delete m_envelope_2;
}

Envelope* EnvelopesContainer::get_envelope_1()
{
    return m_envelope_1;
}

Envelope* EnvelopesContainer::get_envelope_2()
{
    return m_envelope_2;
}